mod branch;
mod infra;
mod registry;
mod safety;
use cli::models::Commands;

pub fn dispatch(command: Commands) {
    match command {
        Commands::Init(args) => {
            println!("Init: {args:#?}");
            // crate::commands::init::run(args);
        }

        Commands::Add(args) => {
            println!("Add: {args:#?}");
        }

        Commands::Remove(args) => {
            println!("Remove: {args:#?}");
        }

        Commands::Status(args) => {
            println!("Status: {args:#?}");
        }

        Commands::Save(args) => {
            println!("Save: {args:#?}");
        }

        Commands::Discard(args) => {
            println!("Discard: {args:#?}");
        }

        Commands::Compact(args) => {
            println!("Compact: {args:#?}");
        }

        Commands::Encrypt(args) => {
            println!("Encrypt: {args:#?}");
        }

        Commands::Log(args) => {
            println!("Log: {args:#?}");
        }

        Commands::Change(args) => {
            println!("Change: {args:#?}");
        }

        Commands::Delete(args) => {
            println!("Delete: {args:#?}");
        }

        Commands::Branch(args) => {
            println!("Branch: {args:#?}");
        }

        Commands::Checkout(args) => {
            println!("Checkout: {args:#?}");
        }

        Commands::Fuse(args) => {
            println!("Fuse: {args:#?}");
        }
    }
}
