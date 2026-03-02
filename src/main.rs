use clap::Parser;
use grove::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Circular => {
            println!("Running circular dependency detection...")
        }
        Commands::Unused => {
            println!("Running unused modules detection...")
        }
    }
}
