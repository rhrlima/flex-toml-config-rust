use base::{load_config, Config};
use plugin;

fn main() {
    load_config::<base::AppConfig>("resources/minimal.toml").unwrap();

    let config: base::AppConfig = load_config("resources/full.toml").unwrap();
    config.to_file("base_test_toml.toml").unwrap();

    load_config::<plugin::AppConfig>("resources/minimal.toml").unwrap();

    let config: plugin::AppConfig = load_config("resources/full.toml").unwrap();
    config.to_file("plugin_test_toml.toml").unwrap();

    ()
}
