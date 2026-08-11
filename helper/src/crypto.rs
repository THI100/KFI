use std::error::Error;

use blake3::Hasher;
use rand::SeedableRng;
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use shake::Shake128;
use shake::digest::{ExtendableOutput, Update, XofReader};

type Errors = Box<dyn Error>;

// ----- Helpers ----- \\

fn sha_hash<D: Digest>(data: &[u8]) -> Vec<u8> {
    let mut hasher = D::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut output = String::with_capacity(bytes.len() * 2);

    for &byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }

    output
}

pub fn encode_hash(algo: &str, input: &str, outsize: &u32) -> Result<String, Errors> {
    let algo_v = if algo == "Sha3" && !matches!(*outsize, 224 | 256 | 384 | 512) {
        "Shake"
    } else {
        algo
    };

    let output_len = (*outsize as usize)
        .checked_div(8)
        .filter(|&size| size > 0 && *outsize % 8 == 0)
        .ok_or("Output size must be a positive multiple of 8.")?;

    let binput = input.as_bytes();

    match algo_v {
        "Blake3" => {
            let mut hasher = Hasher::new();
            hasher.update(binput);

            let mut output = vec![0u8; output_len];
            hasher.finalize_xof().fill(&mut output);

            Ok(to_hex(&output))
        }

        "Kaurea" => Err("This hashing algorithm is currently inactive.".into()),

        "Sha3" => {
            let output = match *outsize {
                224 => sha_hash::<Sha3_224>(binput),
                256 => sha_hash::<Sha3_256>(binput),
                384 => sha_hash::<Sha3_384>(binput),
                512 => sha_hash::<Sha3_512>(binput),
                _ => unreachable!("Invalid SHA-3 size is redirected to SHAKE"),
            };

            Ok(to_hex(&output))
        }

        "Shake" => {
            let mut hasher = Shake128::default();
            hasher.update(binput);

            let mut output = vec![0u8; output_len];
            hasher.finalize_xof().read(&mut output);

            Ok(to_hex(&output))
        }

        _ => Err(format!("Unsupported hashing algorithm: {algo}").into()),
    }
}

pub fn encode_encryption(
    algo: &str,
    input: Vec<u8>,
    key: &str,
    outsize: &u32,
    nonce: Vec<u8>,
) -> Result<String, Errors> {
}

pub fn decode_encryption(
    algo: &str,
    input: Vec<u8>,
    key: &str,
    outsize: &u32,
    nonce: Vec<u8>,
) -> Result<String, Errors> {
}

pub fn generate_nonce(algo: &str, seed: &u128) -> Result<Vec<u8>, Errors> {
    SeedableRng::from_seed(seed);
}
