use clap::{Parser, Subcommand};

mod parser;

#[derive(Parser)]
#[command(
    version,
    about = "Find unused code and circular dependencies in JS/TS projects"
)]
pub struct Cli {
    #[arg(long)]
    verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Circular,
    Unused,
}
