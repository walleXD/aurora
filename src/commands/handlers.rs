use clap::Subcommand;

use super::config::Config;
use std::fmt;

/// Main base commands
#[derive(Subcommand)]
pub enum Command {
    /// Install all plugins
    Load {},
    /// List all plugins
    Ls {},
    /// Add a plugin
    Add {},
    /// Remove a plugin
    Rm {},
    /// Access aurora settings
    Settings {
        #[clap(subcommand)]
        command: SettingsCommand,
    },
}

/// Settings commands
#[derive(Subcommand)]
pub enum SettingsCommand {
    /// List all settings
    Ls {},
}

/// Handle settings commands
pub fn handle_settings_cmds(command: SettingsCommand, config: &mut Config) {
    match command {
        SettingsCommand::Ls {} => {
            println!("The configuration is:");
            println!("{:#?}", config);
        }
    }
}

/// Handle the main commands
pub fn handle_base_cmds(command: Command, _config: &mut Config) {
    match command {
        Command::Load {} => {
            println!("Install all plugins");
        }
        Command::Ls {} => {
            println!("List all plugins");
        }
        Command::Add {} => {
            println!("Add a plugin");
        }
        Command::Rm {} => {
            println!("Remove a plugin");
        }
        _ => {
            panic!("Unknown command");
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
