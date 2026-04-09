# xcell-config

Configuration management for XCell.

## Overview

xcell-config provides configuration management capabilities for the XCell project, including loading, saving, and validating configuration files.

## Features

- Load configuration from TOML files
- Save configuration to TOML files
- Validate configuration values
- Support for nested configuration structures

## Usage

```rust
use xcell_config::{ConfigLoader, NargoConfig};

// Load configuration from a file
let loader = ConfigLoader::new(PathBuf::from("Nargo.toml"));
let config = loader.load()?;

// Access configuration values
if let Some(build_config) = config.build {
    if let Some(prod) = build_config.prod {
        println!("Production mode: {}", prod);
    }
}

// Save configuration
loader.save(&config)?;
```

## Configuration Structure

The configuration file follows the TOML format and supports the following sections:

- `package`: Package information such as name, version, and description
- `build`: Build configuration such as output directory and production mode
- `formatter`: Code formatter configuration
- `linter`: Code linter configuration

## License

MPL-2.0
