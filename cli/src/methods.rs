use crate::models::{self, AddArgs, InitArgs};
use std::path::PathBuf;

type Tokens<'a> = std::str::SplitWhitespace<'a>;

pub fn parse_init(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let vault = parts.next().ok_or("Missing vault name")?;
    let loc = parts.next().ok_or("Missing Path")?;

    Ok(models::Commands::Init(InitArgs {
        vault: vault.to_string(),
        location: PathBuf::from(loc),
    }))
}

pub fn parse_add(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let mut all = false;
    let mut files = Vec::new();

    while let Some(arg) = parts.next() {
        match arg {
            "--all" | "-a" => all = true,
            file => files.push(file.into()),
        }
    }

    Ok(models::Commands::Add(AddArgs {
        all,
        files: if files.is_empty() { None } else { Some(files) },
    }))
}
