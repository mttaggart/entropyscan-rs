use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
mod entropyscan;
use entropyscan::{
    collect_entropies, collect_targets,
    stats::{entropy_outliers, mean, median, variance},
    structs::{FileEntropy, Stats},
};

use serde_json::json;
use tabled::{Table, settings::Style};

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
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// entropy-rs scan
    Scan {
        /// Target path
        #[arg(
            short,
            long,
            value_name = "TARGET",
            help = "Target file or path to scan"
        )]
        target: PathBuf,

        /// Optional minimum entropy threshold
        #[arg(
            short,
            long,
            value_name = "MIN_ENTROPY",
            help = "Minimum entropy to display",
            default_value = "0"
        )]
        min_entropy: Option<f64>,

        #[arg(
            short,
            long,
            value_name = "FORMAT",
            help = "Output format",
            default_value = "table"
        )]
        format: OutputFormat,
    },

    Stats {
        /// Target path
        #[arg(
            short,
            long,
            value_name = "TARGET",
            help = "Target file or path to scan"
        )]
        target: PathBuf,

        #[arg(short, help = "Do not print outliers")]
        no_outliers: bool,

        #[arg(
            short,
            long,
            value_name = "FORMAT",
            help = "Output format",
            default_value = "table"
        )]
        format: OutputFormat,
    },
}

///
/// Our possible output formats
///
#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Table,
    Json,
    Csv,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    // Now that we're using subcommands, all this is in a match!
    match args.command {
        Command::Scan {
            target,
            min_entropy,
            format,
        } => {
            let min_entropy = min_entropy.unwrap();

            let targets = collect_targets(target.to_owned());

            let entropies: Vec<FileEntropy> = collect_entropies(&targets)
                .into_iter()
                .filter(|e| e.entropy >= min_entropy)
                .collect();

            match format {
                OutputFormat::Table => {
                    let mut table = Table::new(entropies);
                    table.with(Style::blank());
                    println!("{table}");
                }
                OutputFormat::Json => {
                    let json = serde_json::to_string_pretty(&entropies).unwrap();
                    println!("{json}");
                }
                OutputFormat::Csv => {
                    println!("path,entropy");
                    for e in entropies {
                        println!("{},{:.3}", e.path.to_str().unwrap(), e.entropy);
                    }
                }
            }

            Ok(())
        }
        Command::Stats {
            target,
            no_outliers,
            format,
        } => {
            let targets = collect_targets(target.to_owned());
            let entropies = collect_entropies(&targets);
            let stats = Stats {
                target,
                total: entropies.len(),
                mean: mean(&entropies).unwrap(),
                median: median(&entropies).unwrap(),
                variance: variance(&entropies).unwrap(),
            };

            match format {
                OutputFormat::Table => {
                    let mut stats_table = Table::new(vec![stats]);
                    stats_table.with(Style::blank());
                    println!("{stats_table}");

                    if !no_outliers {
                        if let Some(outliers) = entropy_outliers(&entropies) {
                            println!("\n========\nOutliers\n========\n");
                            let outliers_table = Table::new(outliers).to_string();
                            println!("{outliers_table}");
                        }
                    }
                }
                OutputFormat::Json => {
                    let json = json!({
                        "stats": stats,
                        "outliers": match no_outliers {
                            true => vec![],
                            false => entropy_outliers(&entropies).unwrap()
                        }
                    });
                    let json_string = serde_json::to_string_pretty(&json).unwrap();
                    println!("{json_string}");
                }
                OutputFormat::Csv => {
                    println!("target,total,mean,median,variance");
                    println!(
                        "{},{},{:.3},{:.3},{:.3}",
                        stats.target.to_str().unwrap(),
                        stats.total,
                        stats.mean,
                        stats.median,
                        stats.variance
                    );
                    if !no_outliers {
                        if let Some(outliers) = entropy_outliers(&entropies) {
                            println!("\n=========\nOutliers\n=========\n");
                            println!("path,entropy");
                            for o in outliers {
                                println!("{},{:.3}", o.path.to_str().unwrap(), o.entropy);
                            }
                        }
                    }
                }
            }

            Ok(())
        }
    }
}
