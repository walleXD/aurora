use clap::{Parser, Subcommand};

/// Simple program to manage hostfiles
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the person to greet
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Install all plugins
    Lock {},
    /// List all plugins
    Ls {},
    /// Add a plugin
    Add {},
    /// Remove a plugin
    Rm {},
}

impl Cli {
    pub fn exec(self) {
        println!("The value of my-arg is ");
    }
}
