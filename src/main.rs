use std::env::args;
use std::fs;

// Max Filesize of 2GB
const MAX_FILESIZE: u64 = 2147483648;

// Chunk size for entropy analysis
const MAX_ENTROPY_CHUNK: usize = 2560000;

fn calculate_entropy(filename: &str) -> Result<f64, String> {
    // Check for ELF

    if let Ok(metadata) = fs::metadata(filename) {
        // Check max size
        if metadata.len() > MAX_FILESIZE {
            return Err("File too large!".to_string());
        }
        // Check whether it's a directory.
        if metadata.is_dir() {
            return Err("This is a directory!".to_string());
        }
    
        if let Ok(file_bytes) = fs::read(filename) {
            let mut entropy = 0.0f64;
            for chunk in file_bytes.chunks(MAX_ENTROPY_CHUNK) {
                let mut frequency: [u32; 256] = [0; 256];
                let mut total_bytes = 0;
        
                for byte in chunk {
                    frequency[*byte as usize] += 1;
                    total_bytes += 1;
                }
        
        
                for count in frequency.iter() {
                    if *count == 0 {
                        continue;
                    }
        
                    let p = *count as f64 / total_bytes as f64;
                    entropy -= p * p.log2();
                }
            }
            Ok(entropy)
    
        } else {
            Err("Couldn't read file!".to_string())
        }

    } else {
        Err("Couldn't get file metadata!".to_string())
    }



}

fn main() -> Result<(),String>{
    if let Some(filename) = args().nth(1) {
        println!("Scanning {filename}");
        let entropy = calculate_entropy(&filename)?;
        println!("Entropy of {filename}: {entropy}");
        Ok(())
    } else {
        panic!("No filename provided!")
    }
}
