use cli::models;
use std::error::Error;
use std::fs;
use std::path::Path;

type Errors = Box<dyn Error>;

pub fn run(args: models::OpenArgs) -> Result<(), Errors> {
    let store_path = Path::new("extdata/store.toml");
    let contents = fs::read_to_string(store_path)?;
    let mut store = contents.parse::<toml_edit::DocumentMut>()?;

    let vaults = store
        .get("vaults")
        .and_then(|item| item.as_table())
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "missing [vaults] table")
        })?;

    if !vaults.contains_key(args.vault.as_str()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("vault does not exist: {}", args.vault),
        )
        .into());
    }

    store["active"] = toml_edit::value(&args.vault);

    fs::write(store_path, store.to_string())?;

    println!("succefully opened the vault: {}", args.vault);
    Ok(())
}
