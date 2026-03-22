//! Plugin system implementation
//!
//! This module provides the core plugin system for XCell, including the Plugin trait
//! and PluginManager for managing plugins.

use std::path::Path;

use xcell_core::{XError, XResult};

/// Workspace manager trait
///
/// This trait defines the interface for workspace manager that plugins can use.
pub trait WorkspaceManager {
    /// Get project configuration
    fn get_config(&self) -> &dyn std::any::Any;
}

/// Plugin trait
///
/// This trait defines the interface for XCell plugins. Plugins can implement this trait
/// to extend XCell's functionality.
pub trait Plugin {
    /// Get plugin name
    fn name(&self) -> &str;

    /// Initialize plugin
    ///
    /// Called when the plugin is loaded, providing access to the workspace manager.
    fn initialize(&mut self, workspace: &dyn WorkspaceManager) -> XResult<()>;

    /// Handle table load event
    ///
    /// Called before a table is loaded.
    fn on_table_load(&self, table: &dyn std::any::Any) -> XResult<()>;

    /// Handle table loaded event
    ///
    /// Called after a table has been loaded.
    fn on_table_loaded(&self, table: &dyn std::any::Any) -> XResult<()>;

    /// Handle export event
    ///
    /// Called before export operation starts.
    fn on_export(&self, workspace: &dyn WorkspaceManager) -> XResult<()>;
}

/// Plugin manager
///
/// Manages the lifecycle and events of plugins.
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    /// Create new plugin manager
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    /// Load plugin
    pub fn load_plugin<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    /// Initialize all plugins
    pub fn initialize(&mut self, workspace: &dyn WorkspaceManager) -> XResult<()> {
        for plugin in &mut self.plugins {
            plugin.initialize(workspace)?;
        }
        Ok(())
    }

    /// Trigger table load event
    pub fn trigger_on_table_load(&self, table: &dyn std::any::Any) -> XResult<()> {
        for plugin in &self.plugins {
            plugin.on_table_load(table)?;
        }
        Ok(())
    }

    /// Trigger table loaded event
    pub fn trigger_on_table_loaded(&self, table: &dyn std::any::Any) -> XResult<()> {
        for plugin in &self.plugins {
            plugin.on_table_loaded(table)?;
        }
        Ok(())
    }

    /// Trigger export event
    pub fn trigger_on_export(&self, workspace: &dyn WorkspaceManager) -> XResult<()> {
        for plugin in &self.plugins {
            plugin.on_export(workspace)?;
        }
        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
