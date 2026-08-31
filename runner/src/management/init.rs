use cli::models;
use serde::Serialize;
use sled;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

type Errors = Box<dyn Error>;

// Structs for config.toml
#[derive(Debug, Serialize)]
struct Config {
    vault: VaultSt,
    defaults: DefSt,
    safety: SafeSt,
}

#[derive(Debug, Serialize)]
struct VaultSt {
    name: String,
    created: String,
    version: u8,
}

#[derive(Debug, Serialize)]
struct DefSt {
    branch: String,
}

#[derive(Debug, Serialize)]
struct SafeSt {
    create_bypass: bool,
    primitive: models::HashAlgo,
    method: models::EncryptionMethod,
}

#[derive(Debug, Serialize)]
struct VaultInfo {
    id: String,
    created: String,
    last_save: Option<String>,
    format: u8,
}

pub fn run(args: models::InitArgs) -> Result<String, Errors> {
    let location = args.location;
    let vault_path = location.join(".vault");
    let bypass_path = location.join(".vaultbypass");

    if location.exists() && !location.is_dir() {
        return Err(format!("Init location is not a directory: {}", location.display()).into());
    }
    fs::create_dir_all(&location)?;

    if vault_path.exists() {
        return Err(format!("Vault already exists: {}", vault_path.display()).into());
    }

    let bypass_created = if bypass_path.exists() {
        if !bypass_path.is_file() {
            return Err(format!("Bypass path is not a file: {}", bypass_path.display()).into());
        }
        false
    } else {
        write_new(&bypass_path, b".vault/\n")?;
        true
    };

    // Create the complete initial vault layout.
    for directory in [
        "branches/trunk/saves",
        "branches/trunk/refs",
        "objects/blobs",
        "objects/trees",
        "objects/manifests",
        "objects/metadata",
        "info/sessions",
        "temp/unaudited_saves",
    ] {
        fs::create_dir_all(vault_path.join(directory))?;
    }

    let created = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .to_string();
    let config = Config {
        vault: VaultSt {
            name: args.vault.clone(),
            created: created.clone(),
            version: 1,
        },
        defaults: DefSt {
            branch: "trunk".into(),
        },
        safety: SafeSt {
            create_bypass: true,
            primitive: args.primitive,
            method: args.method,
        },
    };

    write_new(&vault_path.join("REFER"), b"branches/trunk/HEAD\n")?;
    write_new(
        &vault_path.join("config.toml"),
        toml::to_string_pretty(&config)?.as_bytes(),
    )?;
    write_new(&vault_path.join("branches/trunk/HEAD"), b"Genesis\n")?;

    let vault_info = serde_json::to_string_pretty(&VaultInfo {
        id: args.vault.clone(),
        created: created.clone(),
        last_save: None,
        format: 1,
    })? + "\n";
    write_new(&vault_path.join("info/vault.json"), vault_info.as_bytes())?;
    write_new(
        &vault_path.join("info/statistics.json"),
        b"{\n  \"saves\": 0,\n  \"branches\": 1,\n  \"blobs\": 0,\n  \"encrypted\": false,\n  \"size\": 0\n}\n",
    )?;

    // sled creates and initializes its database at this path.
    let db = sled::open(vault_path.join("index"))?;
    db.flush()?;

    update_store(&args.vault, &location)?;

    if bypass_created {
        println!(
            "Initialized vault at {} and created .vaultbypass",
            location.display()
        );
    } else {
        println!(
            "Initialized vault at {} (.vaultbypass already existed)",
            location.display()
        );
    }

    Ok(format!("succefully initialized the vault: {}", args.vault).to_string())
}

fn write_new(path: &Path, data: &[u8]) -> Result<(), Errors> {
    let mut file = File::create_new(path)?;
    file.write_all(data)?;
    Ok(())
}

fn update_store(vault: &str, location: &Path) -> Result<(), Errors> {
    let store_path = Path::new("extdata/store.toml");
    let contents = fs::read_to_string(store_path)?;
    let mut store = contents.parse::<toml_edit::DocumentMut>()?;

    store["active"] = toml_edit::value(vault);

    let vaults = store["vaults"]
        .or_insert(toml_edit::table())
        .as_table_mut()
        .ok_or("store.toml vaults must be a TOML table")?;

    vaults.remove("playground");
    vaults[vault] = toml_edit::value(location.to_string_lossy().into_owned());

    fs::write(store_path, store.to_string())?;
    Ok(())
}
