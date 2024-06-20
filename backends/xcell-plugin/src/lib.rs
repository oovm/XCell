//! XCell plugin system
//!
//! This crate provides the plugin system for XCell, allowing users to extend XCell's functionality
//! through custom plugins.

/// Plugin module
pub mod plugin;

/// Re-export plugin types
pub use plugin::*;
