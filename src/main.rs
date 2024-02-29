use std::path::PathBuf;
use clap::{Parser, Subcommand};
mod entropyscan;
use entropyscan::{
    collect_targets, 
    collect_entropies,
    structs::{FileEntropy, Stats},
    stats::{
        mean,
        median,
        variance,
        entropy_outliers,
    }
};

use tabled::Table;

/// 
/// Parser config
///
/// Also note that we can know directly create [PathBuf]
/// objects from the args!
///
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {

    /// entropy-rs scan
    Scan {
        /// Target path
        #[arg(short, long, value_name = "TARGET", help="Target file or path to scan")]
        target: PathBuf,
        
        /// Optional minimum entropy threshold
        #[arg(short, long, value_name = "MIN_ENTROPY", help="Minimum entropy to display", default_value = "0")]
        min_entropy: Option<f64>,        
    },

    Stats {

        /// Target path
        #[arg(short, long, value_name = "TARGET", help="Target file or path to scan")]
        target: PathBuf,

        #[arg(short, help = "Do not print outliers")]
        no_outliers: bool
    }


}

fn main() -> Result<(), String> {

    let args = Cli::parse();

    // Now that we're using subcommands, all this is in a match!
    match args.command {
        Command::Scan { target , min_entropy } => {
            let min_entropy = min_entropy.unwrap();
        
            println!("Entropy Threshold: {min_entropy}");
            let targets = collect_targets(PathBuf::from(target.to_owned()));
           
            let entropies: Vec<FileEntropy> = collect_entropies(targets)
                .into_iter()
                .filter(|e| e.entropy >= min_entropy)
                .collect();
            
            let table = Table::new(entropies).to_string();
            println!("{table}");
        
            Ok(())

        },
        Command::Stats { target, no_outliers } => {
            let targets = collect_targets(PathBuf::from(target.to_owned()));
            let entropies = collect_entropies(targets.clone());
            let stats = Stats {
                target,
                total: entropies.len(),
                mean: mean(entropies.clone()).unwrap(),
                median: median(entropies.clone()).unwrap(),
                variance: variance(entropies.clone()).unwrap(),
            };

            let stats_table = Table::new(
                vec![stats]
            )
            .to_string();

            println!("{stats_table}");

            if !no_outliers {
                if let Some(outliers) = entropy_outliers(entropies.clone()) {
                    println!("========\nOutliers\n========");
                    let outliers_table = Table::new(outliers).to_string();
            println!("{outliers_table}");
                }
            }
            Ok(())
        }
    }

}
