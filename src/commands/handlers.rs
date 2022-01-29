use clap::Subcommand;
use owo_colors::OwoColorize;

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
    Rm {
        /// Name of the plugin
        #[clap(short, long)]
        name: String,
    },
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
                name.clone(),
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

            println!("{}", format!("🤖 Added local plugin {}", name).green());
        }

        AddCommand::Remote { name, url } => {
            config.plugins.insert(
                name.clone(),
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

            println!("{}", format!("🤖 Added remote plugin {}", name).green());
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
            println!("{}", "List of all plugins:".green());
            config.plugins.iter().for_each(|(name, _plugin)| {
                println!(
                    "{} {}",
                    name.to_string().green(),
                    _plugin
                        .inactive
                        .map_or("✅".to_string(), |_| "❌".to_string())
                );
            });
        }
        Command::Add { command } => {
            handle_add_commands(command, config);
        }
        Command::Rm { name } => {
            config.plugins.remove(&name);
            println!("{}", format!("🤖 Removed plugin {}", name).red());
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

impl fmt::Display for AddCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for AddCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
