use std::fs;
use std::path::{Path, PathBuf};
use clap::{Parser};

/// 
/// Parser config
///
/// Also note that we can know directly create [PathBuf]
/// objects from the args!
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    /// Target path
    #[arg(short, long, value_name = "TARGET", help="Target file or path to scan")]
    target: PathBuf,

    /// Optional minimum entropy threshold
    #[arg(short, long, value_name = "MIN_ENTROPY", help="Minimum entropy to display", default_value = "0")]
    min_entropy: Option<f64>,
}

// Max Filesize of 2GB
const MAX_FILESIZE: u64 = 2147483648;

// Chunk size for entropy analysis
const MAX_ENTROPY_CHUNK: usize = 2560000;

// const MAGIC_BYTES_LEN: usize = 4;

// const ELF_MAGIC: &str = "7f454c46";

///
/// Calculate the entropy of a single file.
///
/// Uses [MAX_FILESIZE] as a hard limit
/// for what will be scanned.
///
fn calculate_entropy(path: &Path) -> Result<f64, String> {

    // Check for ELF

    // Check max size
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.len() > MAX_FILESIZE {
            return Err("File too large!".to_string());
        }

        if let Ok(file_bytes) = fs::read(path) {
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

///
/// Recursively scans the target path to collect all files to scan
///
fn collect_targets(parent_path: PathBuf) -> Vec<PathBuf> {
    let mut targets = Vec::new();
    if let Ok(dir) = fs::read_dir(parent_path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    targets.extend(collect_targets(path));
                } else {
                    targets.push(path);
                }
            }
        }
    }
    targets
}


fn main() -> Result<(), String> {

    let args = Cli::parse();
    let min_entropy = args.min_entropy.unwrap();
    let target = args.target;

    println!("Entropy Threshold: {min_entropy}");
    let targets = collect_targets(PathBuf::from(target.to_owned()));
    for target in targets {
        let entropy = calculate_entropy(&PathBuf::from(target.to_owned()))?;
        // Only print when entropy is above threshold
        if entropy >= min_entropy {
            println!("{target:?}: {entropy}");
        }
    }
    Ok(())
}
