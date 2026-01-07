use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "harv")]
#[command(about = "Personal cognitive operating system")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize HARV
    Init,

    /// Ingest new thoughts
    Ingest,

    /// Process raw thoughts
    Process,
} 

fn main() {
    let cli = Cli::parse();

    match  cli.command {
        Commands::Init => {
            println!("Initializing HARV...");
        }
        Commands::Ingest => {
            println!("Ingesting thoughts...");
        }
        Commands::Process => {
            println!("Processing thoughts...");
        }
    }
}