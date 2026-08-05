use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub mod crypto;

type Errors = Box<dyn std::error::Error>;

// Struct for store.toml

#[derive(Debug, Serialize)]
pub struct Store {
    active: String,
    vaults: HashMap<String, String>,
}

fn sure_extdata() -> io::Result<()> {
    let extdata = Path::new("extdata");

    if !extdata.exists() {
        fs::create_dir(extdata)?;
    }

    Ok(())
}

pub fn create_store() -> Result<(), Errors> {
    sure_extdata()?;

    let store_path = Path::new("extdata/store.toml");

    if store_path.exists() {
        return Ok(());
    }

    let mut vaults = HashMap::new();
    vaults.insert("playground".to_string(), "./extdata/playground".to_string());

    let store = Store {
        active: "playground".to_string(),
        vaults,
    };

    let mut file = File::create(store_path)?;
    file.write_all(toml::to_string(&store)?.as_bytes())?;

    Ok(())
}

pub fn read_store() -> Result<PathBuf, Errors> {
    let store_path = Path::new("extdata/store.toml");
    let contents = fs::read_to_string(store_path)?;
    let store = contents.parse::<toml_edit::DocumentMut>()?;

    let active = store
        .get("active")
        .and_then(toml_edit::Item::as_str)
        .ok_or("Missing or invalid 'active' value in extdata/store.toml")?;

    let vaults = store
        .get("vaults")
        .and_then(toml_edit::Item::as_table)
        .ok_or("Missing or invalid [vaults] table in extdata/store.toml")?;

    let vault_path = vaults
        .get(active)
        .and_then(toml_edit::Item::as_str)
        .ok_or_else(|| format!("No vault path found for active vault '{active}'"))?;

    Ok(PathBuf::from(vault_path))
}

pub fn get_safety_config(origin: impl AsRef<Path>) -> Result<(String, String), Errors> {
    let config_path = origin.as_ref().join(Path::new(".vault/config.toml"));
    let contents = fs::read_to_string(config_path)?;
    let config = contents.parse::<toml_edit::DocumentMut>()?;

    let safety = config
        .get("safety")
        .and_then(toml_edit::Item::as_table)
        .ok_or("Missing or invalid [safety] table in vault config.toml")?;

    let primitive = safety
        .get("primitive")
        .and_then(toml_edit::Item::as_str)
        .ok_or("Missing or invalid 'primitive' value in [safety]")?
        .to_owned();

    let method = safety
        .get("method")
        .and_then(toml_edit::Item::as_str)
        .ok_or("Missing or invalid 'method' value in [safety]")?
        .to_owned();

    Ok((primitive, method))
}
