use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::plugin::Plugin;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(serialize_with = "toml::ser::tables_last")]
    plugins: HashMap<String, Plugin>,
}
