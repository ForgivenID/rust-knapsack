use std::path::Path;
use anyhow::Result;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub hash: String,
    pub chunks: Vec<ChunkMetadata>,
    pub duration: f64,
    pub codec: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub hash: String,
    pub size: usize,
    pub order: usize,
}

pub struct
VideoProcessor;

impl VideoProcessor {
    pub fn prepare_video<P: AsRef<Path>>(path: P) -> Result<VideoMetadata> {
        // For now, we'll just hash the file and create a single chunk
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let hash = Sha256::digest(&buffer);
        let hash_hex = hex::encode(hash);

        // Create a single chunk for the entire file
        let chunk = ChunkMetadata {
            hash: hash_hex.clone(),
            size: buffer.len(),
            order: 0,
        };

        Ok(VideoMetadata {
            hash: hash_hex,
            chunks: vec![chunk],
            duration: 0.0, // We'll need to implement this later
            codec: "mp4".to_string(),
            title: "Untitled".to_string(),
            description: "No description".to_string(),
        })
    }
}