use std::path::{Path, PathBuf};
use clap::{Parser};
mod entropyscan;
use entropyscan::{
    collect_targets, 
    calculate_entropy,
    structs::FileEntropy
};

/// 
/// Parser config
///
/// Also note that we can know directly create [PathBuf]
/// objects from the args!
///
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

fn main() -> Result<(), String> {

    let args = Cli::parse();
    let min_entropy = args.min_entropy.unwrap();
    let target = args.target;

    println!("Entropy Threshold: {min_entropy}");
    let targets = collect_targets(PathBuf::from(target.to_owned()));
    // Store entropies for analysis
    let mut entropies: Vec<FileEntropy> = Vec::with_capacity(targets.len());
    for target in targets {
        // Create the entropy entries and push them
        let file_entropy = calculate_entropy(&PathBuf::from(target.to_owned()))?;
        entropies.push(file_entropy);
    }

    Ok(())
}
