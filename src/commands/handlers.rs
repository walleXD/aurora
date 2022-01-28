use clap::Subcommand;

use super::{config::Config, plugin::Plugin};
use std::fmt;

/// Main base commands
#[derive(Subcommand)]
pub enum Command {
    /// Install all plugins
    Load {},
    /// List all plugins
    Ls {},
    /// Add a plugin
    Add {
        #[clap(subcommand)]
        command: AddCommand,
    },
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
pub enum AddCommand {
    /// Add a local plugin
    Local {
        /// Name of the plugin
        #[clap(short, long)]
        name: String,

        /// path for host file
        #[clap(short, long)]
        path: String,
    },
    /// Add a local plugin
    Remote {
        /// Name of the plugin
        #[clap(short, long)]
        name: String,

        /// uri to host file
        #[clap(short, long)]
        url: String,
    },
    /// Add a plugin from github
    Github {},
    /// Add a plugin from git repo
    Git {},
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

/// Handle add commands
pub fn handle_add_commands(command: AddCommand, config: &mut Config) {
    match command {
        AddCommand::Local { name, path } => {
            config.plugins.insert(
                name,
                Plugin {
                    local: Some(path),
                    inactive: None,
                    remote: None,
                    github: None,
                    git: None,
                    dir: None,
                    file: None,
                },
            );
        }

        AddCommand::Remote { name, url } => {
            config.plugins.insert(
                name,
                Plugin {
                    remote: Some(url),
                    inactive: None,
                    github: None,
                    local: None,
                    git: None,
                    dir: None,
                    file: None,
                },
            );
        }

        AddCommand::Github {} => {
            println!("🙈 Not implemented yet");
        }

        AddCommand::Git {} => {
            println!("🙈 Not implemented yet");
        }
    }
}

/// Handle the main commands
pub fn handle_base_cmds(command: Command, config: &mut Config) {
    match command {
        Command::Load {} => {
            println!("Install all plugins");
        }
        Command::Ls {} => {
            println!("List all plugins");
        }
        Command::Add { command } => {
            handle_add_commands(command, config);
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
