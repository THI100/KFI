use std::path::PathBuf;
use std::str::FromStr;

// ---------- Enums ---------- \\

#[derive(Debug)]
pub enum Commands {
    // Vault Management
    Init(InitArgs),   // Create a new Vault in a folder
    Open(OpenArgs),   // Open an existing Vault
    Clone(CloneArgs), // Create a copy of a Vault
    Destroy(DesArgs), // Permanently remove a Vault
    // File Tracking
    Add(AddArgs),     // Stage files for the next save
    Remove(RemArgs),  // Untrack staged files
    Status(StatArgs), // Show the current workspace status
    Diff(DiffArgs),   // Compare files from saves or saves
    // Saves
    Save(SaveArgs),    // Create an audited save
    Restore(RestArgs), // Restore files from a previous save
    Discard(DissArgs), // Remove an unaudited save
    Log(LogArgs),      // List audited save history
    Inspect(InsArgs),  // Show detailed information about a save
    // Safety
    Encrypt(EcArgs),   // Encrypt Vaults, saves, or selected contents
    Compact(CompArgs), // Compress saves or the entire Vault
    Verify(VeriArgs),  // Verify snapshot or Vault integrity and blockchain health
    Export(ExpoArgs),  // Export a protected Vault archive
    // Branching
    Branch(BranchArgs), // Create or remove branches
    Switch(SwitchArgs), // Change the active branch
    Fuse(FuseArgs),     // Merge two branches
}

#[derive(Debug)]
pub enum HashAlgo {
    Blake3,
    Kaurea,
    Sha3,
}

impl FromStr for HashAlgo {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "blake3" | "blake" => Ok(HashAlgo::Blake3),
            "kaurea" => Ok(HashAlgo::Kaurea),
            "sha3" | "sha" => Ok(HashAlgo::Sha3),
            _ => Err("Unknown hash algorithm"),
        }
    }
}

#[derive(Debug)]
pub enum EncryptionMethod {
    Aes,
    ChaCha20,
    ChaCha20Poly1305,
    AesGcm,
}

impl FromStr for EncryptionMethod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "aes" => Ok(EncryptionMethod::Aes),
            "chacha" | "chacha20" => Ok(EncryptionMethod::ChaCha20),
            "chachapoly" | "chacha20poly" | "chacha20poly1305" => {
                Ok(EncryptionMethod::ChaCha20Poly1305)
            }
            "aesgcm" => Ok(EncryptionMethod::AesGcm),
            _ => Err("Unknown Encryption algorithm"),
        }
    }
}

#[derive(Debug)]
pub enum InternalObject {
    Vault,
    Save,
}

impl FromStr for InternalObject {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "vault" => Ok(InternalObject::Vault),
            "save" | "commit" => Ok(InternalObject::Save),
            _ => Err("Unknown Internal Object"),
        }
    }
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
    pub save: Option<String>,
}

#[derive(Debug)]
pub struct SaveArgs {
    pub message: String,
    pub primitive: HashAlgo,
    pub flags: Option<Vec<SaveFlags>>,
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
    pub otype: InternalObject,
    pub id: String,
    pub method: EncryptionMethod,
    pub plus_security: bool, // Use Argon2 to hash special files, such as a .env or password file
    pub output: Option<PathBuf>,
    pub key: Option<String>, // Used with method, Automaticly is applied Argon2
    pub apply_ps: Option<Vec<PathBuf>>, // Apply plus_security to those files
}

#[derive(Debug)]
pub struct LogArgs {
    pub count: Option<u16>, // amount of saves that will be shown
    pub filters: Option<Vec<LogFilters>>,
}

#[derive(Debug)]
pub struct OpenArgs {
    pub vault: String,
}

#[derive(Debug)]
pub struct DesArgs {
    pub vault: String,
}

#[derive(Debug)]
pub struct BranchArgs {
    pub branch: String,
    pub flag: Option<BranchFlags>,
}

#[derive(Debug)]
pub struct SwitchArgs {
    pub branch: String,
}

#[derive(Debug)]
pub struct FuseArgs {
    pub branch1: String,
    pub branch2: String,
    pub message: Option<String>,
    pub flags: Option<Vec<FuseFlags>>,
}

#[derive(Debug)]
pub struct CloneArgs {
    pub dest_vault: String,
    pub source_vault: String,
}

#[derive(Debug)]
pub struct DiffArgs {
    pub save1: Option<String>,
    pub save2: Option<String>,
    pub files: Option<Vec<PathBuf>>,
}

#[derive(Debug)]
pub struct RestArgs {
    pub save: String,
    pub overwrite: bool,
    pub files: Option<Vec<PathBuf>>,
}

#[derive(Debug)]
pub struct InsArgs {
    pub save: String,
    pub flags: Option<Vec<InsFlags>>,
}

#[derive(Debug)]
pub struct VeriArgs {
    pub otype: InternalObject,
    pub id: String,
}

#[derive(Debug)]
pub struct ExpoArgs {
    pub vault: String,
    pub extreme_safety: bool,
    pub save: Option<String>,
    pub destination: Option<PathBuf>,
}

// ---------- Enums for flags ---------- \\

#[derive(Debug)]
pub enum FuseFlags {
    ByPass,
    InteruptionForSave,
    Abort,
}

impl FromStr for FuseFlags {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "-bp" | "--by-pass" => Ok(FuseFlags::ByPass),
            "-i" | "--interupt" | "--manual-inspect" => Ok(FuseFlags::InteruptionForSave),
            "--abort" => Ok(FuseFlags::Abort),
            _ => Err("Unknown Internal Object"),
        }
    }
}

#[derive(Debug)]
pub enum BranchFlags {
    Delete,
    New,
}

impl FromStr for BranchFlags {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "-d" | "--delete" | "--del" => Ok(BranchFlags::Delete),
            "-n" | "--new" => Ok(BranchFlags::New),
            _ => Err("Unknown Internal Object"),
        }
    }
}

#[derive(Debug)]
pub enum LogFilters {
    AlphabeticOrder,
    TimeOrder,
    OnlyEncypted,
    OnlyCompacted,
}

impl FromStr for LogFilters {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "--alphabetic" | "-ao" | "-abc" => Ok(LogFilters::AlphabeticOrder),
            "-oc" | "--only-compacted" => Ok(LogFilters::OnlyCompacted),
            "-oe" | "--only-encrypted" => Ok(LogFilters::OnlyEncypted),
            "--time" | "-to" => Ok(LogFilters::TimeOrder),
            _ => Err("Unknown Internal Object"),
        }
    }
}

#[derive(Debug)]
pub enum InsFlags {
    Detailed,
    Simplified,
    Status,
}

impl FromStr for InsFlags {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "-det" | "--detailed" => Ok(InsFlags::Detailed),
            "-sim" | "--simplified" => Ok(InsFlags::Simplified),
            "-stat" | "--status" => Ok(InsFlags::Status),
            _ => Err("Unknown Internal Object"),
        }
    }
}

#[derive(Debug)]
pub enum SaveFlags {
    AllowEmpty,
    Sign,
}

impl FromStr for SaveFlags {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "--allow-empty" | "-ae" => Ok(SaveFlags::AllowEmpty),
            "--sign" | "-s" => Ok(SaveFlags::Sign),
            _ => Err("Unknown Internal Object"),
        }
    }
}
