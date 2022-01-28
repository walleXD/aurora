mod config;
mod handlers;
mod plugin;

use clap::Parser;
use confy::{load, store, ConfyError};

use self::config::Config;
use self::handlers::{handle_base_cmds, handle_settings_cmds, Command};

/// Simple program to manage hostfiles
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the person to greet
    #[clap(subcommand)]
    command: Command,
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
