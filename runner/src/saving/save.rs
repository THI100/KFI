use cli::models;
use std::error::Error;

type Errors = Box<dyn Error>;

pub fn run(args: models::SaveArgs) -> Result<String, Errors> {
    return Err(format!(
        "Work in progress, but this is the command and arguments: Save: {args:#?}"
    )
    .into());
}
