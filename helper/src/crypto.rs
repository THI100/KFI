use std::error::Error;
type Errors = Box<dyn Error>;
use blake3::{Hasher};
use sha3::{Digest};
use shake::{Shake128};
use shake::digest::{Update, ExtendableOutput, XofReader};

// ----- Helpers ----- \\

fn sha_hash<D: Digest>(data: &[u8]) -> Vec<u8> {
    let mut hasher = D::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}


pub fn encode_hash(algo: &str, input: &str, outsize: &u32) -> Result<(), Errors> {

    let mut algo_v = algo;

    if !matches!(outsize, 224 | 256 | 384 | 512) && algo == "Sha3" {
        algo_v = "Shake";
    }

    match algo_v {
        "Blake3" => {
            let binput = input.as_bytes();

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
            return Err("This hashing algorithm is currently inactive.".into());
        }

        "Sha3" => {
            let binput = input.as_bytes();
            let mut out;

            match outsize {
                224 => out = sha_hash<Sha3_224>(binput)
                256 => out = sha_hash<Sha3_256>(binput)
                384 => out = sha_hash<Sha3_384>(binput)
                512 => out = sha_hash<Sha3_512>(binput)
            }


        }
        "Shake" => {
            let binput = input.as_bytes();

            let mut hasher = Shake128::default();

            hasher.update(binput);

            let mut hex_string = hasher.finalize_xof();

            return hex_string;
        }
    }
}

pub fn encode_encryption() -> Result<String, Errors> {}
