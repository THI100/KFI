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

        Commands::Open(args) => {
            println!("Open: {args:#?}");
        }

        Commands::Destroy(args) => {
            println!("Destroy: {args:#?}");
        }

        Commands::Branch(args) => {
            println!("Branch: {args:#?}");
        }

        Commands::Switch(args) => {
            println!("Switch: {args:#?}");
        }

        Commands::Fuse(args) => {
            println!("Fuse: {args:#?}");
        }

        Commands::Clone(args) => {
            println!("Clone: {args:#?}");
        }

        Commands::Diff(args) => {
            println!("Diff: {args:#?}");
        }

        Commands::Restore(args) => {
            println!("Restore: {args:#?}");
        }

        Commands::Inspect(args) => {
            println!("Inspect: {args:#?}");
        }

        Commands::Verify(args) => {
            println!("Verify: {args:#?}");
        }

        Commands::Export(args) => {
            println!("Export: {args:#?}");
        }
    }
}
