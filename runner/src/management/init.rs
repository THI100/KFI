use cli::models;
use serde::Serialize;
use sled;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

type Errors = Box<dyn Error>;

// Structs for config.toml
#[derive(Debug, Serialize)]
struct Config {}

pub fn run(args: models::InitArgs) -> Result<(), Errors> {
    return Err(format!(
        "Work in progress, but this is the command and arguments: Init: {args:#?}"
    )
    .into());

    // Initialization
    let pathv: PathBuf = args.location.join(PathBuf::from("/.vault"));
    let pathf: PathBuf = args.location.join(PathBuf::from("/.vaultbypass"));

    // Checks
    if pathf.is_file() {
        println!("Warning: .vaultbypass file is existent");
    } else {
        File::create_new(pathf)?;
    }

    if pathv.is_dir() {
        return Err(".vault folder is existent".into());
    }

    // Create folders
    fs::create_dir_all(pathv)?;
    fs::create_dir(pathv.join(PathBuf::from("/branches")))?;
    fs::create_dir_all(pathv.join(PathBuf::from("/trunk/saves")))?;
    fs::create_dir_all(pathv.join(PathBuf::from("/trunk/refs")))?;
    fs::create_dir(pathv.join(PathBuf::from("/objects")))?;

    // Create files
    let mut act = File::create_new(pathv.join(PathBuf::from("ACT")))?; // Actual branch, actual save
    let mut conf = File::create_new(pathv.join(PathBuf::from("config.toml")))?; // Config file
    let db: sled::Db = sled::open(pathv.join(PathBuf::from("index"))).unwrap(); // Index File

    // Edit and add data to files
    act.write_all("Branch: Trunk \n Save: Genesis".as_bytes())?;
}
