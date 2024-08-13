use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::Write;
use prover::{
    aggregator::Prover as BatchProver, zkevm::Prover as ChunkProver, BatchProof, BatchProvingTask,
    BundleProvingTask, ChunkProvingTask,
};

pub fn save_chunk_proof_to_file(chunk_proof: &prover::ChunkProof, file_path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string(chunk_proof)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}