use std::path::PathBuf;

// ---------- Enums ---------- \\

#[derive(Debug)]
pub enum Commands {
    Init(InitArgs),      // Starts a new Vault of a certain folder
    Add(AddArgs),        // Add new files to track
    Remove(RemArgs),     // Remove tracked files
    Status(StatArgs),    // Get the atual status of the save
    Save(SaveArgs),      // Save the actual Added files
    Discard(DissArgs),   // Delete a certain save
    Compact(CompArgs),   // Compact a certain save to .zip or the whole Vault
    Encrypt(EcArgs),     // Encrypt a certain save and its contents
    Log(LogArgs),        // Gives the history of saves
    Revert(RevArgs),     // Revert to a certain save or file and delete history (only for the file).
    Change(ChangeArgs),  // Changes to another Vault
    Delete(DelArgs),     // Delete the Vault
    Branch(BranchArgs),  // Create or delete a Vault branch
    Checkout(ChOutArgs), // Changes between branches
    Fuse(FuseArgs),      // Fuse two branches together
}

#[derive(Debug)]
pub enum HashAlgo {
    Blake3,
    Kaurea,
    Sha3,
}

#[derive(Debug)]
pub enum EncryptionMethod {
    Aes,
    ChaCha20,
    ChaCha20Poly1305,
    AesGcm,
}

// ---------- Arguments ---------- \\

#[derive(Debug)]
pub struct InitArgs {
    pub vault: String,
    pub location: PathBuf,
}

#[derive(Debug)]
pub struct AddArgs {
    pub all: bool,
    pub files: Option<Vec<PathBuf>>,
}

#[derive(Debug)]
pub struct RemArgs {
    pub files: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct StatArgs {
    pub save: String,
}

#[derive(Debug)]
pub struct SaveArgs {
    pub message: String,
    pub primitive: HashAlgo,
    pub flags: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct DissArgs {
    pub save: String,
}

#[derive(Debug)]
pub struct CompArgs {
    pub all: bool,
    pub save: Option<String>,
}

#[derive(Debug)]
pub struct EcArgs {
    pub save: String,
    pub method: EncryptionMethod,
    pub key: Option<String>, // Used with method, Automaticly is applied Argon2
    pub plus_security: bool, // Use Argon2 to hash special files, such as a .env or password file
    pub apply_ps: Option<Vec<PathBuf>>, // Apply plus_security to those files
}

#[derive(Debug)]
pub struct LogArgs {
    pub count: u8, // amount of saves that will be shown
    pub filters: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct RevArgs {
    pub save: String,
    pub file: Option<PathBuf>,
}

#[derive(Debug)]
pub struct ChangeArgs {
    pub vault: String,
}

#[derive(Debug)]
pub struct DelArgs {
    pub vault: String,
}

#[derive(Debug)]
pub struct BranchArgs {
    pub branch: String,
    pub flag: Option<String>,
}

#[derive(Debug)]
pub struct ChOutArgs {
    pub branch: String,
}

#[derive(Debug)]
pub struct FuseArgs {
    pub branch1: String,
    pub branch2: String,
    pub flags: Option<Vec<String>>,
}
