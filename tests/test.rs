use base::{BaseConfig, load_config};
use plugin::PluginConfig;

#[test]
fn test_base_full_config() {
    let config = base::AppConfig {
        base: BaseConfig {
            name: "Full Config".to_string(),
            version: "1.0.0".to_string(),
            host: "myhost.com".to_string(),
            port: 9999,
        },
    };

    let loaded_config: base::AppConfig = load_config("resources/full.toml")
        .expect("Could not load TOML file.");

    assert_eq!(config, loaded_config);
}

#[test]
fn test_base_minimal_config() {
    let config = base::AppConfig {
        base: BaseConfig {
            name: "Minimal Config".to_string(),
            version: "1.0.0".to_string(),
            host: "localhost".to_string(),
            port: 8080,
        },
    };

    let loaded_config: base::AppConfig = load_config("resources/minimal.toml")
        .expect("Could not load TOML file.");

    assert_eq!(config, loaded_config);
}

#[test]
fn test_plugin_full_config() {
    let config = plugin::AppConfig {
        base: BaseConfig {
            name: "Full Config".to_string(),
            version: "1.0.0".to_string(),
            host: "myhost.com".to_string(),
            port: 9999,
        },
        plugin: PluginConfig {
            plugin_name: "MyPlugin".to_string(),
            plugin_version: "2.2.2".to_string(),
            param1: "I am required".to_string(),
            param2: Some("I am optional".to_string()),
        }
    };

    let loaded_config: plugin::AppConfig = load_config("resources/full.toml")
        .expect("Could not load TOML file.");

    assert_eq!(config, loaded_config);
}

#[test]
fn test_plugin_minimal_config() {
    let config = plugin::AppConfig {
        base: BaseConfig {
            name: "Minimal Config".to_string(),
            version: "1.0.0".to_string(),
            host: "localhost".to_string(),
            port: 8080,
        },
        plugin: PluginConfig {
            plugin_name: "MyPlugin".to_string(),
            plugin_version: "1.1.1".to_string(),
            param1: "I am required".to_string(),
            param2: None,
        }
    };

    let loaded_config: plugin::AppConfig = load_config("resources/minimal.toml")
        .expect("Could not load TOML file.");

    assert_eq!(config, loaded_config);
}

#[test]
fn test_nonexistent_file() {
    let config  = load_config::<base::AppConfig>("");
    assert!(config.is_err(), "Expected an error for invalid TOML");
}

#[test]
fn test_invalid_file() {
    let config  = load_config::<base::AppConfig>("resources/invalid.toml");
    assert!(config.is_err(), "Expected an error for invalid TOML");
}
