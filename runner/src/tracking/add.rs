use cli::models;
use helper::read_store;
use std::error::Error;
use walkdir::WalkDir;

type Errors = Box<dyn Error>;

pub fn run(args: models::AddArgs) -> Result<(), Errors> {
    // return Err(
    //     format!("Work in progress, but this is the command and arguments: Add: {args:#?}").into(),
    // );

    let alive = read_store();
    let paths = Vec::new();

    if args.all == true {
        for path in WalkDir::new(alive).into_iter().filter_map(|e| e.ok()) {
            paths.push(path);
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
