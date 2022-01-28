use clap::{Parser, Subcommand};
use confy::{load, store, ConfyError};
use core::panic;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Simple program to manage hostfiles
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the person to greet
    #[clap(subcommand)]
    command: Command,
}

/// Main base commands
#[derive(Subcommand)]
enum Command {
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
enum SettingsCommand {
    /// List all settings
    Ls {},
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

#[derive(Debug, Serialize, Deserialize)]
enum PluginKind {
    Local,
    Git,
    Github,
    Remote,
}

#[derive(Debug, Serialize, Deserialize)]
struct Plugin {
    kind: PluginKind,
    path: Option<String>,
    dir: Option<String>,
    file: String,
}

impl Default for Plugin {
    fn default() -> Self {
        Plugin {
            kind: PluginKind::Local,
            path: None,
            dir: None,
            file: "hosts".to_string(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    #[serde(serialize_with = "toml::ser::tables_last")]
    plugins: HashMap<String, Plugin>,
}

/// Handle settings commands
fn handle_settings_cmds(command: SettingsCommand, config: &mut Config) {
    match command {
        SettingsCommand::Ls {} => {
            println!("The configuration is:");
            println!("{:#?}", config);
        }
    }
}

/// Handle the main commands
fn handle_base_cmds(command: Command, _config: &mut Config) {
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

impl Cli {
    /// Run the program
    pub fn exec(self) -> Result<(), ConfyError> {
        // load settings from a config file
        let mut cfg: Config = load(env!("CARGO_PKG_NAME"))?;
        let cfg_ref = &mut cfg;

        // handle commands
        match self.command {
            Command::Settings { command } => {
                handle_settings_cmds(command, cfg_ref);
            }
            _ => {
                handle_base_cmds(self.command, cfg_ref);
            }
        }

        // store settings from a config file
        store(env!("CARGO_PKG_NAME"), cfg_ref)?;

        Ok(())
    }
}
