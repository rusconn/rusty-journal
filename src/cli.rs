use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "Rusty Journal",
    version,
    about = "A command line to-do app written in Rust",
    long_about = None
)]
pub struct CommandLineArgs {
    #[command(subcommand)]
    pub action: Action,

    #[arg(short, long)]
    pub journal_file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        text: String,
    },
    /// Remove an entry from the journal file by position.
    Done { position: usize },
    /// List all tasks in the journal file.
    List,
}
