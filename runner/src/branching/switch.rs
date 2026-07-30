use cli::models;
use std::error::Error;

type Errors = Box<dyn Error>;

pub fn run(args: models::SwitchArgs) -> Result<(), Errors> {
    return Err(format!(
        "Work in progress, but this is the command and arguments: Switch: {args:#?}"
    )
    .into());
}
