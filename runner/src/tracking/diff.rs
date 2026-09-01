use cli::models;
use std::error::Error;

type Errors = Box<dyn Error>;

pub fn run(args: models::DiffArgs) -> Result<(), Errors> {
    return Err(format!(
        "Work in progress, but this is the command and arguments: Diff: {args:#?}"
    )
    .into());
}
