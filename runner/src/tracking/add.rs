use cli::models;
use helper::read_store;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

type Errors = Box<dyn Error>;

pub fn run(args: models::AddArgs) -> Result<(), Errors> {
    // return Err(
    //     format!("Work in progress, but this is the command and arguments: Add: {args:#?}").into(),
    // );

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
            // a line that starts with * is all files that has the suffix: .rs, .html, .py (line 44 to 48)
            // a line that starts with # will be ignored (line 36 to 39)
            // a line that starts with / is a folder all of its contents will be excluded (line 49 to 50)
            // a line that starts with " " or . is a file (line 51 to 54)

            let rule = tmp.trim_end_matches(['\r', '\n']);

            if rule.is_empty() || rule.trim_start().starts_with('#') {
                tmp.clear();
                continue;
            }

            let rule = rule.trim();
            paths.retain(|path| {
                let relative = path.strip_prefix(&alive).unwrap_or(path);
                let ignored = if let Some(suffix) = rule.strip_prefix('*') {
                    relative
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| extension == suffix.trim_start_matches('.'))
                } else if let Some(folder) = rule.strip_prefix('/') {
                    relative.starts_with(Path::new(folder.trim_end_matches('/')))
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

    // Continue here...
    // I know the code above is totally red, it will be fixed soon.

    Ok(())
}
