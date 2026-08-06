use std::error::Error;
type Errors = Box<dyn Error>;

pub fn encode_hash(algo: &str, input: Vec<u8>, outsize: Option<u32>) -> Result<String, Errors> {
    match algo {
        "Blake3" => {}
        "Kaurea" => {}
        "Sha3" => {}
    }
}

pub fn encode_encryption() -> Result<String, Errors> {}

pub fn encode_stream_hash(algo: &str, input: Vec<u8>, outsize: Option<u32>) -> Result<String, Errors> {
    match algo {
        "Blake3" => {}
        "Kaurea" => {}
        "Sha3" => {}
    }
}
