use std::path::PathBuf;

// ---------- Enums ---------- \\

pub enum Commands {
    Init(InitArgs),     // Starts a new Vault of a certain folder
    Add(AddArgs),       // Add new files to track
    Remove(RemArgs),    // Remove tracked files
    Status(StatArgs),   // Get the atual status of the save
    Save(SaveArgs),     // Save the actual Added files
    Discard(DissArgs),  // Delete a certain save
    Compact(CompArgs),  // Compact a certain save to .zip or the whole Vault
    Encrypt(EcArgs),    // Encrypt a certain save and its contents
    Log(LogArgs),       // Gives the history of saves
    Revert(RevArgs),    // Revert to a certain save or file and delete history (only for the file).
    Change(ChangeArgs), // Changes to another Vault
    Delete(DelArgs),    // Delete the Vault
}

pub enum HashAlgo {
    Blake3,
    Kaurea,
    Sha3,
}

pub enum EncryptionMethod {
    Aes,
    ChaCha20,
    ChaCha20Poly1305,
    AesGcm,
}

// ---------- Arguments ---------- \\

pub struct InitArgs {
    pub vault: String,
    pub location: PathBuf,
}

pub struct AddArgs {
    pub all: bool,
    pub files: Option<Vec<PathBuf>>,
}

pub struct RemArgs {
    pub files: Vec<PathBuf>,
}

pub struct StatArgs {
    pub save: String,
}

pub struct SaveArgs {
    pub message: String,
    pub primitive: HashAlgo,
    pub flags: Option<Vec<String>>,
}

pub struct DissArgs {
    pub save: String,
}

pub struct CompArgs {
    pub all: bool,
    pub save: Option<String>,
}

pub struct EcArgs {
    pub save: String,
    pub method: EncryptionMethod,
    pub key: Option<String>, // Used with method, Automaticly is applied Argon2
    pub plus_security: bool, // Use Argon2 to hash special files, such as a .env or password file
    pub apply_ps: Option<Vec<PathBuf>>, // Apply plus_security to those files
}

pub struct LogArgs {
    pub count: u8, // amount of saves that will be shown
    pub filters: Option<Vec<String>>,
}

pub struct RevArgs {
    pub save: String,
    pub file: Option<PathBuf>,
}

pub struct ChangeArgs {
    pub vault: String,
}

pub struct DelArgs {
    pub vault: String,
}

// ---------- Functions ---------- \\

pub fn parse<'a, T>(req: &'a T) -> Result<Command, CliError> {
    let mut parts = vec![];
    parts = req.split(" ");
}
