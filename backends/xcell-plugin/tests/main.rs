//! Tests for xcell-plugin

use xcell_core::{WorkspaceManager, XResult};
use xcell_plugin::{Plugin, PluginManager};

/// Test plugin implementation
struct TestPlugin {
    name: String,
}

impl TestPlugin {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl Plugin for TestPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn initialize(&mut self, _workspace: &WorkspaceManager) -> XResult<()> {
        Ok(())
    }

    fn on_table_load(&self, _table: &dyn xcell_core::x_table::table::TableReader) -> XResult<()> {
        Ok(())
    }

    fn on_table_loaded(&self, _table: &dyn xcell_core::x_table::table::TableReader) -> XResult<()> {
        Ok(())
    }

    fn on_export(&self, _workspace: &WorkspaceManager) -> XResult<()> {
        Ok(())
    }
}

#[test]
fn test_plugin_manager() {
    let mut manager = PluginManager::new();
    let plugin = TestPlugin::new("test-plugin");

    // Load plugin
    manager.load_plugin(plugin);

    // Test initialization (should succeed)
    let workspace = WorkspaceManager::new();
    assert!(manager.initialize(&workspace).is_ok());

    // Test events (should succeed)
    // Note: We can't test table events without a real TableReader implementation
    assert!(manager.trigger_on_export(&workspace).is_ok());
}
