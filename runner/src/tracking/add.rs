use cli::models;
use helper::read_store;
use helper::{crypto::encode_hash, get_safety_config};
use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, mpsc};
use walkdir::WalkDir;

type Errors = Box<dyn Error>;

pub fn run(args: models::AddArgs) -> Result<(), Errors> {
    let alive = read_store()?;
    let mut paths = Vec::new();

    if args.all == true {
        for path in WalkDir::new(&alive).into_iter().filter_map(|e| e.ok()) {
            paths.push(path.path().to_path_buf());
        }

        let bypass = File::open(alive.join(".vaultbypass"))?;
        let mut reader = BufReader::new(bypass);
        let mut tmp = String::new();
        while reader.read_line(&mut tmp)? > 0 {
            // remove an path from the paths, based on a line, considering:

            let rule = tmp.trim_end_matches(['\r', '\n']);

            // a line that starts with # will be ignored (line 29 to 32)
            if rule.is_empty() || rule.trim_start().starts_with('#') {
                tmp.clear();
                continue;
            }

            let rule = rule.trim();
            paths.retain(|path| {
                let relative = path.strip_prefix(&alive).unwrap_or(path);
                // a line that starts with * is all files that has the suffix: .rs, .html, .py (line 38 to 42)
                let ignored = if let Some(suffix) = rule.strip_prefix('*') {
                    relative
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| extension == suffix.trim_start_matches('.'))
                // a line that starts with / is a folder all of its contents will be excluded (line 44 to 45)
                } else if let Some(folder) = rule.strip_prefix('/') {
                    relative.starts_with(Path::new(folder.trim_end_matches('/')))
                // a line that starts with " " or . is a file (line 47 to 50)
                } else {
                    relative == Path::new(rule.trim_start_matches('.'))
                        || relative == Path::new(rule)
                };

                !ignored
            });

            tmp.clear();
        }
    } else {
        if let Some(files) = args.files {
            for file in files {
                paths.push(alive.join(file));
            }
        }
    }

    let temp_dir = alive.join(".vault/objects/blobs");
    if !temp_dir.is_dir() {
        return Err("The blob directory does not exist!".into());
    }
    let safety = get_safety_config(&alive)?;
    let primitive = safety.0;

    let saves_dir = alive.join(".vault/temp/unaudited_saves");
    if !saves_dir.is_dir() {
        return Err("The unaudited saves directory does not exist!".into());
    }
    let tmp_save_dir = saves_dir.join("tmp");
    fs::create_dir_all(&tmp_save_dir)?;

    let db = sled::open(alive.join(".vault/index"))?;
    let inserted_hashes = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

    let (sender, receiver) = mpsc::channel::<Result<(std::path::PathBuf, String), String>>();
    let producer = sender.clone();
    // Loop 2 must be able to observe channel closure after Loop 1 finishes.
    // Keep only the producer clone used by Loop 1; otherwise `receiver` waits
    // forever because this original sender remains alive until the end of run.
    drop(sender);

    // Loop 1 (Hashing and naming, missing sled db integration)

    let (hash_result, write_result) = rayon::join(
        || -> Result<(), String> {
            let db = db.clone();
            let inserted_hashes = Arc::clone(&inserted_hashes);
            let result = paths
                .par_iter()
                .enumerate()
                .filter(|(_, origin)| origin.is_file())
                .try_for_each(|(thread_number, origin)| -> Result<(), String> {
                    let temp_snapshot_path = temp_dir.join(format!("snap_t_{thread_number}.tmp"));
                    let result = (|| {
                        let src_file = File::open(origin).map_err(|error| error.to_string())?;
                        let mut reader = BufReader::new(src_file);
                        let dest_file =
                            File::create(&temp_snapshot_path).map_err(|error| error.to_string())?;
                        let mut writer = BufWriter::new(dest_file);
                        let mut buffer = [0; 8192];
                        let mut snapshot = Vec::new();

                        // Store the original file name as the first line of the snapshot.
                        let original_name = origin
                            .file_name()
                            .ok_or_else(|| "the source path has no file name".to_string())?;
                        let header = format!("{}\n\n", original_name.to_string_lossy());
                        writer
                            .write_all(header.as_bytes())
                            .map_err(|error| error.to_string())?;
                        snapshot.extend_from_slice(header.as_bytes());

                        loop {
                            let bytes_read = reader
                                .read(&mut buffer)
                                .map_err(|error| error.to_string())?;
                            if bytes_read == 0 {
                                break;
                            }
                            writer
                                .write_all(&buffer[..bytes_read])
                                .map_err(|error| error.to_string())?;
                            snapshot.extend_from_slice(&buffer[..bytes_read]);
                        }

                        writer.flush().map_err(|error| error.to_string())?;
                        let hash = encode_hash(&primitive, &snapshot, &128)
                            .map_err(|error| error.to_string())?;
                        let snapshot_path = temp_dir.join(format!("{hash}.bin"));

                        // Index the snapshot before publishing its final name. If any
                        // worker fails, all inserted keys are removed below.
                        let key = hash.as_bytes().to_vec();
                        db.insert(&key, snapshot_path.to_string_lossy().as_bytes())
                            .map_err(|error| error.to_string())?;
                        inserted_hashes
                            .lock()
                            .map_err(|error| error.to_string())?
                            .push(key);

                        fs::rename(&temp_snapshot_path, snapshot_path)
                            .map_err(|error| error.to_string())?;
                        producer
                            .send(Ok((origin.clone(), hash)))
                            .map_err(|error| error.to_string())?;
                        Ok::<(), String>(())
                    })();

                    if result.is_err() {
                        let _ = fs::remove_file(&temp_snapshot_path);
                    }
                    result
                });
            drop(producer);
            result
        },
        // Loop 2, which makes tree sturcture in temp/
        || -> Result<(), String> {
            let mut created_dirs = HashSet::new();
            for message in receiver {
                let (folder, hash) = message?;
                let relative = folder
                    .strip_prefix(&alive)
                    .map_err(|error| error.to_string())?;
                let relative_parent = relative.parent().unwrap_or(Path::new(""));
                let destination = tmp_save_dir.join(relative_parent);

                if created_dirs.insert(destination.clone()) {
                    fs::create_dir_all(&destination).map_err(|error| error.to_string())?;
                    File::create(destination.join("source")).map_err(|error| error.to_string())?;
                }
                let mut source = fs::OpenOptions::new()
                    .append(true)
                    .open(destination.join("source"))
                    .map_err(|error| error.to_string())?;
                writeln!(source, "{hash},").map_err(|error| error.to_string())?;
            }
            Ok(())
        },
    );

    if let Err(error) = hash_result {
        if let Ok(keys) = inserted_hashes.lock() {
            for key in keys.iter() {
                let _ = db.remove(key);
            }
            let _ = db.flush();
        }
        return Err(error.into());
    }
    db.flush()?;
    write_result.map_err(|error| -> Errors { error.into() })?;

    let paths_str = paths
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!(
        "successfully added the following files to the active vault: {}",
        paths_str
    );

    Ok(())
}
