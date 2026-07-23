pub enum Commands {
    Add(AddArgs),
    Save(SaveArgs),
    Encrypt(EcArgs),
    Log(LogArgs),
    Drop,
}

// Arguments

pub struct AddArgs {
    pub all: bool,
    pub files: Option<Vec<String>>,
}

pub struct SaveArgs {
    pub message: String,
    pub primative: String,
    pub flags: Option<Vec<String>>,
}

pub struct EcArgs {
    pub method: String,
}

pub struct LogArgs {
    pub size: u8,
    pub filters: Option<Vec<String>>,
}

// pub fn parse<I>(args: I) -> Result<Command, CliError>
// where
//     I: IntoIterator,
// {

// }
