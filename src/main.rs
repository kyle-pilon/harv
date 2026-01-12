use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod db;

#[derive(Parser)]
#[command(name = "mysticrhythm")]
#[command(about = "Personal cognitive operating system")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize Mystic Rhythm
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
            println!("Initializing Mystic Rhythms...");

            // Determine database path
            let db_path = get_db_path();
            println!("Database will be created at: {}", db_path.display());

            // Create database and initialize schema
            match db::Database::new(&db_path) {
                Ok(db) => {
                    match db.init_schema() {
                        Ok(_) => println!("✓ Database initialized successfully"),
                        Err(e) => eprintln!("Error initializing schema: {}", e),
                    }
                }
                Err(e) => eprintln!("Error creating database: {}", e),
            }
        }
        Commands::Ingest => {
            println!("Ingesting thoughts...");
        }
        Commands::Process => {
            println!("Processing thoughts...");
        }
    }
}

/// Get the path where the database should live
fn get_db_path() -> PathBuf {
    // Get user's local app data directory
    let local_app_data = std::env::var("LOCALAPPDATA")
        .expect("LOCALAPPDATA environment variable not set");

    let harv_dir = PathBuf::from(local_app_data).join("mysticrhythm");

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&harv_dir)
        .expect("Failed to  create mysticrhythm directory");

    harv_dir.join("mysticrhythm.db")
}