use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
mod entropyscan;
use entropyscan::{
    collect_targets, 
    collect_entropies,
    structs::FileEntropy,
    stats::{
        mean,
        median,
        variance,
        entropy_outliers,
        interquartile_range
    }
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
        no_outliers: Option<bool>
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
           
            let entropies = collect_entropies(targets);
            
            println!("PATH\tENTROPY");
            for e in entropies {
                if e.entropy >= min_entropy {
                    println!("{}\t{:.3}", e.path.to_str().unwrap(), e.entropy)
                }
            }
        
            Ok(())

        },
        Command::Stats { target, no_outliers } => {
            let targets = collect_targets(PathBuf::from(target.to_owned()));
            let entropies = collect_entropies(targets);
            println!("Statistics for {}", target.to_str().unwrap());
            println!("Mean Entropy: {:.3}", mean(entropies.clone()).unwrap());
            println!("Median Entropy: {:.3}", median(entropies.clone()).unwrap());
            println!("Variance Entropy: {:.3}", variance(entropies.clone()).unwrap());
            println!("IQR: {:?}", interquartile_range(entropies.clone()).unwrap());
            if let None = no_outliers {
                if let Some(outliers) = entropy_outliers(entropies.clone()) {
                    println!("Outliers\n========");
                    for o in outliers {
                        println!("{}\t{:.3}", o.path.to_str().unwrap(), o.entropy)
                    }
                }
            }
            Ok(())
        }
    }

}
