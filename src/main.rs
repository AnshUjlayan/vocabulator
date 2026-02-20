mod audio;
mod core;
mod db;
mod seed;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use db::init_db;
use seed::seed_from_file;

#[derive(Parser)]
#[command(name = "vocabulator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Seed { file: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = init_db("vocab.db")?;

    match cli.command {
        Some(Commands::Seed { file }) => {
            seed_from_file(&conn, &file)?;
            println!("Database seeded successfully.");
        }
        None => {
            ui::run::run()?;
        }
    }

    Ok(())
}
