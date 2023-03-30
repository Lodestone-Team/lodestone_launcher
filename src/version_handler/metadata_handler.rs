use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub current_version: String,
    pub last_updated: String,
}

pub fn read_metadata() -> Metadata {
    let path = Path::new("metadata.json");
    let mut contents = String::new();

    match File::open(path) {
        Ok(mut file) => {
            if let Err(e) = file.read_to_string(&mut contents) {
                println!("Error reading metadata file: {}", e);
            }
        }
        Err(e) => {
            println!("Error opening metadata file: {}", e);
        }
    }
    let metadata = match serde_json::from_str(&contents) {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing metadata file: {}", e);
            Metadata {
                current_version: "0.0.0".to_string(),
                last_updated: "N/A".to_string(),
            }
        }
    };
    return metadata;
}

pub fn update_metadata(metadata: &Metadata) -> Result<(), io::Error> {
    let mut file = File::create("metadata.json")?;
    let json = serde_json::to_string(metadata)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
