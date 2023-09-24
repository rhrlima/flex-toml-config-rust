use base::{load_config, Config};
use plugin;

fn main() {
    load_config::<base::AppConfig>("resources/minimal.toml").unwrap();
    load_config::<base::AppConfig>("resources/full.toml").unwrap();
    
    load_config::<plugin::AppConfig>("resources/minimal.toml").unwrap();
    let config: plugin::AppConfig = load_config("resources/full.toml").unwrap();

    config.to_file("test_toml.toml").unwrap()
}
