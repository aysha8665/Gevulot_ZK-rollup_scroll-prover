use serde_json;
use std::fs::File;
use std::io::{self, Read};

pub fn load_chunk_proof_from_file(file_path: &str) -> io::Result<prover::ChunkProof> {
    let mut file = File::open(file_path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let chunk_proof: prover::ChunkProof = serde_json::from_str(&json)?;
    Ok(chunk_proof)
}