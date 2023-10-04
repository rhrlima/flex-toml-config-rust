use serde::{self, Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

pub trait Config: Default + Debug + Serialize + for<'de> Deserialize<'de> {

    fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    fn to_string(&self) -> String {
        match self.to_toml() {
            Ok(str) => str,
            Err(err) => {
                println!("Failed to convert config to String with error: {}", err);
                "".to_string()
            }
        }
    }

    fn to_file(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let toml_str = self.to_string();
        let path = Path::new(file_name);
        
        let mut file = File::create(&path)?;
        file.write_all(toml_str.as_bytes())?;
    
        println!("Config saved to: {}", path.display());
        Ok(())
    }
}

pub fn load_config<T: Config>(file_path: &str) -> Result<T, Box<dyn Error>> {
    let contents = read_to_string(file_path)?;
    let parsed: T = toml::from_str(&contents)?;
    println!("{:?}", parsed);
    Ok(parsed)
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct BaseConfig {
    pub name: String,
    pub version: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
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
