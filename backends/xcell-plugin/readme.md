# xcell-plugin

XCell plugin system

## Overview

The xcell-plugin crate provides the plugin system for XCell, allowing users to extend XCell's functionality through custom plugins.

## Features

- Plugin trait for creating custom plugins
- PluginManager for managing plugin lifecycle and events
- Event system for table loading and export operations

## Usage

```rust
use xcell_plugin::{Plugin, PluginManager};
use xcell_core::{XResult, WorkspaceManager};

struct MyPlugin {
    name: String,
}

impl MyPlugin {
    fn new() -> Self {
        Self { name: "my-plugin".to_string() }
    }
}

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn initialize(&mut self, workspace: &WorkspaceManager) -> XResult<()> {
        // Initialize plugin
        Ok(())
    }
    
    fn on_table_load(&self, table: &dyn xcell_core::x_table::table::TableReader) -> XResult<()> {
        // Handle table load event
        Ok(())
    }
    
    fn on_table_loaded(&self, table: &dyn xcell_core::x_table::table::TableReader) -> XResult<()> {
        // Handle table loaded event
        Ok(())
    }
    
    fn on_export(&self, workspace: &WorkspaceManager) -> XResult<()> {
        // Handle export event
        Ok(())
    }
}

// Usage
let mut manager = PluginManager::new();
let plugin = MyPlugin::new();
manager.load_plugin(plugin);

// Initialize plugins
let workspace = WorkspaceManager::new();
manager.initialize(&workspace)?;
```

## Dependencies

- xcell-core: Core functionality for XCell
- tracing: Logging support
- serde: Serialization support

## License

MPL-2.0
