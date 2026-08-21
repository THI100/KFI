use cli::models;
use helper::read_store;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use walkdir::WalkDir;

type Errors = Box<dyn Error>;

pub fn run(args: models::AddArgs) -> Result<(), Errors> {
    // return Err(
    //     format!("Work in progress, but this is the command and arguments: Add: {args:#?}").into(),
    // );

    let alive = read_store()?;
    let paths = Vec::new();

    if args.all == true {
        for path in WalkDir::new(alive).into_iter().filter_map(|e| e.ok()) {
            paths.push(alive.join(path));
        }

        let bypass = File::open(alive.join(".vaultbypass"))?;
        let reader = BufReader::new(bypass);
        let mut tmp = String::new();
        while reader.read_line(&mut tmp)? > 0 {
            // remove an path from the paths, considering:
            // that starts with * is all files that has the suffix: .rs, .html, .py
            // that starts with # will be ignored
            // that starts with / is a folder all of its contents will be excluded
            // that starts with " " or . is a file

            tmp.clear();
        }
    } else {
        if !args.files.map_or(true, |v| v.is_empty()) {
            for file in args.files {
                paths.push(alive.join(file));
            }
        }
    }

    // Continue here...
    // I know the code above is totally red, it will be fixed soon.
}
