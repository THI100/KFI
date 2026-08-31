use cli::models;
use std::error::Error;

type Errors = Box<dyn Error>;

pub fn run(args: models::BranchArgs) -> Result<String, Errors> {
    return Err(format!(
        "Work in progress, but this is the command and arguments: Branch: {args:#?}"
    )
    .into());
}
