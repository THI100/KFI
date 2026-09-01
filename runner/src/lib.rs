use cli::models::Commands;
mod branching;
mod management;
mod safety;
mod saving;
mod tracking;

pub fn dispatch(command: Commands) {
    let result = match command {
        // Vault Management \\
        Commands::Init(args) => management::init::run(args),
        Commands::Open(args) => management::open::run(args),
        Commands::Clone(args) => management::clone::run(args),
        Commands::Destroy(args) => management::destroy::run(args),

        // File Tracking \\
        Commands::Add(args) => tracking::add::run(args),
        Commands::Remove(args) => tracking::remove::run(args),
        Commands::Status(args) => tracking::status::run(args),
        Commands::Diff(args) => tracking::diff::run(args),

        // Saves \\
        Commands::Save(args) => saving::save::run(args),
        Commands::Restore(args) => saving::restore::run(args),
        Commands::Discard(args) => saving::discard::run(args),
        Commands::Log(args) => saving::log::run(args),
        Commands::Inspect(args) => saving::inspect::run(args),

        // Safety \\
        Commands::Encrypt(args) => safety::encrypt::run(args),
        Commands::Compact(args) => safety::compact::run(args),
        Commands::Verify(args) => safety::verify::run(args),
        Commands::Export(args) => safety::export::run(args),

        // Branching \\
        Commands::Branch(args) => branching::branch::run(args),
        Commands::Switch(args) => branching::switch::run(args),
        Commands::Fuse(args) => branching::fuse::run(args),
    };

    // Handle any errors returned by the subcommand
    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
