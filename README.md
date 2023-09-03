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
