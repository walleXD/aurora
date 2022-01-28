use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub local: Option<String>,
    pub remote: Option<String>,
    pub github: Option<String>,
    pub git: Option<String>,
    pub dir: Option<String>,
    pub file: Option<String>,
    pub inactive: Option<bool>,
}
