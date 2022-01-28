use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PluginKind {
    Local,
    Remote,
    Github,
    Git,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
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
