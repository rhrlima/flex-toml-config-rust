use serde::{Deserialize, Serialize};
use base::{BaseConfig, Config};

// Plugin configuration
#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct PluginConfig {
    pub plugin_name: String,
    pub plugin_version: String,
    pub param1: String,
    pub param2: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct AppConfig {
    pub base: BaseConfig,
    pub plugin: PluginConfig,
}

impl Config for AppConfig {}
