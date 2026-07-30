use cli::models::Commands;
mod branching;
mod management;
mod safety;
mod saving;
mod tracking;

pub fn dispatch(command: Commands) {
    match command {
        // Vault Management \\
        Commands::Init(args) => management::init::run(args),

        Commands::Open(args) => {
            println!("Open: {args:#?}");
        }

        Commands::Clone(args) => {
            println!("Clone: {args:#?}");
        }

        Commands::Destroy(args) => {
            println!("Destroy: {args:#?}");
        }

        // File Tracking \\
        Commands::Add(args) => {
            println!("Add: {args:#?}");
        }

        Commands::Remove(args) => {
            println!("Remove: {args:#?}");
        }

        Commands::Status(args) => {
            println!("Status: {args:#?}");
        }

        Commands::Diff(args) => {
            println!("Diff: {args:#?}");
        }

        // Saves \\
        Commands::Save(args) => {
            println!("Save: {args:#?}");
        }

        Commands::Restore(args) => {
            println!("Restore: {args:#?}");
        }

        Commands::Discard(args) => {
            println!("Discard: {args:#?}");
        }

        Commands::Log(args) => {
            println!("Log: {args:#?}");
        }

        Commands::Inspect(args) => {
            println!("Inspect: {args:#?}");
        }

        // Safety \\
        Commands::Encrypt(args) => {
            println!("Encrypt: {args:#?}");
        }

        Commands::Compact(args) => {
            println!("Compact: {args:#?}");
        }

        Commands::Verify(args) => {
            println!("Verify: {args:#?}");
        }

        Commands::Export(args) => {
            println!("Export: {args:#?}");
        }

        // Branching \\
        Commands::Branch(args) => {
            println!("Branch: {args:#?}");
        }

        Commands::Switch(args) => {
            println!("Switch: {args:#?}");
        }

        Commands::Fuse(args) => {
            println!("Fuse: {args:#?}");
        }
    }
}
