use cli::models;
use helper::read_store;
use helper::{crypto::encode_hash, get_safety_config};
use std::collections::HashSet;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;
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

    // Loop 1 (Hashing and naming, missing sled db integration)

    for origin in &paths {
        if !origin.is_file() {
            continue;
        }

        let temp_dir = alive.join(".vault/objects/blobs");
        if !temp_dir.is_dir() {
            return Err("The blob directory does not exist!".into());
        }
        let temp_snapshot_path = temp_dir.join("snap.tmp");

        let src_file = File::open(origin)?;
        let mut reader = BufReader::new(src_file);

        let dest_file = File::create(&temp_snapshot_path)?;
        let mut writer = BufWriter::new(dest_file);

        let mut buffer = [0; 8192];
        let mut snapshot = Vec::new();

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            writer.write_all(&buffer[..bytes_read])?;
            snapshot.extend_from_slice(&buffer[..bytes_read]);
        }

        writer.flush()?;

        let safety = get_safety_config(&alive)?;
        let primitive: &str = &safety.0;

        let hash = encode_hash(primitive, &snapshot, &128)?;
        let snapshot_path = temp_dir.join(format!("{hash}.bin"));

        // Rename the completed temporary snapshot to its cryptographic name.
        fs::rename(temp_snapshot_path, snapshot_path)?;
    }

    // Loop 2 (FOlder structure)

    let saves_dir = alive.join(".vault/temp/unaudited_saves");
    if !saves_dir.is_dir() {
        return Err("The unaudited saves directory does not exist!".into());
    }

    let tmp_save_dir = saves_dir.join("tmp");
    fs::create_dir_all(&tmp_save_dir)?;

    let mut created_dirs = HashSet::new();

    for folder in paths {
        if !folder.is_file() {
            continue;
        }

        let relative = folder.strip_prefix(&alive)?;
        let relative_parent = relative.parent().unwrap_or(Path::new(""));
        let destination = tmp_save_dir.join(relative_parent);

        if created_dirs.insert(destination.clone()) {
            fs::create_dir_all(&destination)?;

            let source_file = destination.join("source");
            let mut source = File::create(source_file)?;
            source.write_all(b"0,\n")?;
        }
    }

    // Loop 1 and 2 are missing an mscp channel for transmission of the hash values and parallelism

    Ok(())
}
