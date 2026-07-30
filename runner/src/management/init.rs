use cli::models;
use sled;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

type Err = String;

pub fn run(args: models::InitArgs) -> Result<(), Err> {
    println!(
        "Work in progress, but this is the command and arguments: Init: {:?}",
        args
    );

    fs::create_dir_all(args.location)?;
    fs::create_dir(args.location.join(PathBuf::from("/branches")))?;
    fs::create_dir_all(args.location.join(PathBuf::from("/trunk/saves")))?;
    fs::create_dir_all(args.location.join(PathBuf::from("/trunk/refs")))?;
    fs::create_dir(args.location.join(PathBuf::from("/objects")))?;

    let mut act = File::create_new(args.location.join(PathBuf::from("ACT")));
    let mut conf = File::create_new(args.location.join(PathBuf::from("config.toml")));
    let db: sled::Db = sled::open(args.location.join(PathBuf::from("index"))).unwrap();
}
