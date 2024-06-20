#![warn(missing_docs)]
//! # XCell WASI 绑定
//!
//! 这个库为 XCell 提供 WASI 绑定，
//! 使其可以在各种环境中使用。

// #[allow(warnings)]
// mod bindings;
//
// use crate::bindings::exports::xcell_wasi::Guest;
use serde_json::Value as JsonValueNode;
use xcell_core::{WorkspaceManager, XResult};

// struct XCellWasi;
//
// struct XCellInstance {
//     inner: std::sync::Mutex<Option<WorkspaceManager>>,
// }
//
// impl Guest for XCellWasi {
//     type Vm = XCellInstance;
//
//     fn hello() -> String {
//         "Hello from XCell WASI!".to_string()
//     }
//
//     fn process_table(source: Option<String>, output: Option<String>) -> Result<(), String> {
//         println!("Processing table from {:?} to {:?}", source, output);
//         // 这里将实现具体的表格处理逻辑
//         Ok(())
//     }
//
//     fn validate_table(source: Option<String>) -> Result<(), String> {
//         println!("Validating table in {:?}", source);
//         // 这里将实现具体的表格验证逻辑
//         Ok(())
//     }
// }
//
// impl bindings::exports::xcell_wasi::GuestVm for XCellInstance {
//     fn new() -> Self {
//         XCellInstance { inner: std::sync::Mutex::new(None) }
//     }
//
//     fn process_data(&self, data: Option<String>) -> Result<String, String> {
//         let mut instance = self.inner.lock().unwrap();
//         // 这里将实现具体的数据处理逻辑
//         Ok("Processed data successfully".to_string())
//     }
// }
//
// bindings::export!(XCellWasi with_types_in bindings);
