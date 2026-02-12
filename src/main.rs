mod db;
mod seed;

use anyhow::Result;
use clap::{Parser, Subcommand};
use db::init_db;
use seed::seed_from_file;

#[derive(Parser)]
#[command(name = "vocabulator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Seed { file: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = init_db("vocab.db")?;

    match cli.command {
        Commands::Seed { file } => {
            seed_from_file(&conn, &file)?;
            println!("Database seeded successfully.");
        }
    }

    Ok(())
}
