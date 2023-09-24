# Flex TOML Configs in Rust

A simple example on how to make extendable configurations from TOML files.

## Base Config

Example of a common config file you can have for your project.

```toml
[base]
param1 = value1
param2 = value2
# optional_param_with_default
# optional_param_no_default
```
## Extended Config

Another example that extends the `base` config from the previous example, and includes a new `plugin` section.

```toml
[base]
param1 = value1
param2 = value2
# optional_param_with_default = value3
# optional_param_no_default = value4

[plugin]
param1 = value1
param2 = value2
# optional_param_with_default = value3
# optional_param_no_default = value4
```

## Example

TOML configurations are loaded into Rust structs. And these structs are composed of `Config` and `Block` structs:

### Base Config

Here, `BaseConfig` defines a `block` with fields we want:

```rust
#[derive(Default, Debug, Deserialize, PartialEq)]
pub struct BaseConfig {
    pub name: String,
    pub version: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}
```

We provide default values for optional fields:

```rust
fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    8080
}
```

Finally we define `AppConfig` as our `config`, which uses the `BaseConfig` as a section.

```rust
#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct AppConfig {
    pub base: BaseConfig,
}
```

### Reading TOMLs

We can use `load_config` to read a TOML file and return our `AppConfig` struct. However, first we need `AppConfig` to implement `Config` which is the generic type accepted by the `load_config` function.

```rust
# the Config trait
pub trait Config: Default + Debug + Serialize + for<'de> Deserialize<'de> {}

# Implementing the trait for other configs
impl Config for AppConfig {}
```

Thenm, you can call `load_config`, using type annotation to specify the config type.

```rust
let config: plugin::AppConfig = load_config("path/to/config.toml").unwrap();
```

### Saving configs to TOML file

We can call `to_file` from a parsed config, providing a name, to save it as a TOML file.

```rust
config.to_file("test_toml.toml")
```
