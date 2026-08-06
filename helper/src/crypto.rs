use std::error::Error;
type Errors = Box<dyn Error>;
use blake3::{Hasher, }
use sha3
use shake

pub fn encode_hash(algo: &str, input: &str, outsize: u32) -> Result<(), Errors> {

    let mut algo_v = algo;

    if !matches!(outsize, 224 | 256 | 384 | 512) && algo == "Sha3" {
        algo_v = "Shake";
    }

    match algo {
        "Blake3" => {
            binput = input.as_bytes();

            let mut hasher = Hasher::new();
            hasher.update(binput);

            let mut output_reader = hasher.finalize_xof();

            let mut custom_out = [0u8; outsize];

            output_reader.fill(&mut custom_output);

            let hex_string: String = custom_output
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect();

            return hex_string
        }
        "Kaurea" => {
            // Wait for the adaptation to take variable output sizes
        }
        "Sha3" => {}
        "Shake" => {}
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
