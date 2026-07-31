use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

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

pub fn create_store() -> Result<(), Box<dyn std::error::Error>> {
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
