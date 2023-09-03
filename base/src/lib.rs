extern crate toml;

use serde;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Debug;
use std::fs::read_to_string;

pub trait Config: Default + Debug+ for<'de> Deserialize<'de> {}

pub fn load_config<T: Config>(file_path: &str) -> Result<T, Box<dyn Error>> {
    let contents = read_to_string(file_path)?;
    let parsed: T = toml::from_str(&contents)?;
    println!("{:?}", parsed);
    Ok(parsed)
}

#[derive(Default, Debug, Deserialize, PartialEq)]
pub struct BaseConfig {
    pub name: String,
    pub version: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Default, Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub base: BaseConfig,
}

impl Config for AppConfig {}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    8080
}