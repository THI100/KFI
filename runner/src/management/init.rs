use cli::models;
use sled;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn run(args: models::InitArgs) -> Result {
    println!(
        "Work in progress, but this is the command and arguments: Init: {:?}",
        args
    );

    let path: PathBuf = args.location.join(PathBuf::from("/.vault"));

    if path.is_dir() {
        return ".vault folder is existent";
    }

    // Create folders
    fs::create_dir_all(path)?;
    fs::create_dir(path.join(PathBuf::from("/branches")))?;
    fs::create_dir_all(path.join(PathBuf::from("/trunk/saves")))?;
    fs::create_dir_all(path.join(PathBuf::from("/trunk/refs")))?;
    fs::create_dir(path.join(PathBuf::from("/objects")))?;

    // Create files
    let mut act = File::create_new(path.join(PathBuf::from("ACT")))?; // Actual branch, actual save
    let mut conf = File::create_new(path.join(PathBuf::from("config.toml")))?; // Config file
    let db: sled::Db = sled::open(path.join(PathBuf::from("index"))).unwrap(); // Index File

    // Edit and add data to files
    act.write_all("Branch: Trunk \n Save: Genesis".as_bytes())?;
    conf.write()
}
