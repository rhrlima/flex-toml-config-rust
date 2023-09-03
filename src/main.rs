use base;
use base::load_config;
use plugin;

fn main() {
    load_config::<base::AppConfig>("resources/full.toml").unwrap();
    load_config::<base::AppConfig>("resources/minimal.toml").unwrap();
    println!("---");
    load_config::<plugin::AppConfig>("resources/full.toml").unwrap();
    load_config::<plugin::AppConfig>("resources/minimal.toml").unwrap();
}
