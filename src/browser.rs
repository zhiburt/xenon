use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub fn default_sessions_per_driver() -> u32 {
    1
}

pub fn default_max_sessions() -> u32 {
    5
}

#[derive(Debug, Clone, Deserialize)]
pub struct BrowserConfig {
    name: String,
    version: Option<String>,
    os: Option<String>,
    driver_path: PathBuf,
    args: Option<Vec<String>>,
    #[serde(default = "default_sessions_per_driver")]
    sessions_per_driver: u32,
    #[serde(default = "default_max_sessions")]
    max_sessions: u32,
}

impl BrowserConfig {
    pub fn name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn driver_path(&self) -> &Path {
        &self.driver_path.as_path()
    }

    pub fn args(&self) -> &Option<Vec<String>> {
        &self.args
    }

    pub fn sessions_per_driver(&self) -> u32 {
        self.sessions_per_driver
    }

    pub fn max_sessions(&self) -> u32 {
        self.max_sessions
    }

    /// Does this browser match the capabilities we are searching for?
    /// Browser name must match.
    /// For browser version and platform, the following rules apply:
    /// 1. If the required browser version or platform is specified,
    ///    the system will only consider it a match if those are both
    ///    known and identical.
    /// 2. If the actual version or platform is not specified on the browser
    ///    object, it is considered unknown and thus will only match if the
    ///    version or platform is not required.
    pub fn matches_capabilities(&self, capabilities: &Capabilities) -> bool {
        if self.name.to_lowercase() != capabilities.browser_name().to_lowercase() {
            return false;
        }

        if let Some(required_version) = capabilities.browser_version() {
            if !required_version.is_empty() {
                match &self.version {
                    Some(v) => {
                        if v != required_version {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }

        if let Some(required_os) = capabilities.platform_name() {
            let required_os = required_os.to_lowercase();
            if required_os.to_lowercase() != "any" {
                match &self.os {
                    Some(os) => {
                        if os.to_lowercase() != required_os {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }

        true
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserMatch {
    browser_name: String,
    browser_version: Option<String>,
    platform_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    always_match: BrowserMatch,
}

impl Capabilities {
    pub fn browser_name(&self) -> &str {
        &self.always_match.browser_name
    }

    pub fn browser_version(&self) -> &Option<String> {
        &self.always_match.browser_version
    }

    pub fn platform_name(&self) -> &Option<String> {
        &self.always_match.platform_name
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct W3CCapabilities {
    /// The W3C capabilities object, used to match browser/version/OS etc.
    pub capabilities: serde_json::Value,
    /// All of the additional browser-specific capabilities such as extra arguments etc.
    #[serde(default)]
    pub desired_capabilities: serde_json::Value,
}
