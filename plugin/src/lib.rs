use serde::{Deserialize, Serialize};
use base::{BaseConfig, Config};
use std::collections::HashMap;
use strfmt;

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

impl Config for AppConfig {
    fn to_string(&self) -> String {
        let parameters = get_parameters(&self);

        match strfmt::strfmt(TOML_TEMPLATE, &parameters) {
            Ok(parsed) => parsed,
            Err(err) => {
                println!("Failed to convert config to String with error: {}", err);
                "".to_string()
            }
        }
    }
}

fn get_parameters(config: &AppConfig) -> HashMap<String, String> {
    let mut parameters: HashMap<String, String> = HashMap::new();
    parameters.insert("base_name".to_string(), config.base.name.to_string());
    parameters.insert("base_version".to_string(), config.base.version.to_string());
    parameters.insert("plugin_name".to_string(), config.plugin.plugin_name.to_string());
    parameters.insert("plugin_version".to_string(), config.plugin.plugin_version.to_string());
    parameters.insert("plugin_param1".to_string(), config.plugin.param1.to_string());
    parameters
}

const TOML_TEMPLATE: &str = r#"# Plugin Template

# Base section
[base]
# app name
name = "{base_name}"
# app version
version = "{base_version}"
# Using defaults
# host = "myhost.com"
# port = 9999

# Plugin section
[plugin]
plugin_name = "{plugin_name}"
plugin_version = "{plugin_version}"
# required parameter
param1 = "{plugin_param1}"
# optional parameter
# param2 = "I am optional"
"#;
