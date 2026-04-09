# xcell-wasi

XCell WASI bindings for WebAssembly support.

## Overview

xcell-wasi provides WebAssembly System Interface (WASI) bindings for the XCell project, allowing XCell functionality to be executed in WebAssembly environments.

## Features

- WASI bindings for XCell functionality
- Support for WebAssembly modules
- Integration with WASM runtimes
- Cross-platform compatibility

## Usage

```rust
use xcell_wasi::analyze_table;

// Analyze a table in a WebAssembly environment
let result = analyze_table(&table_data)?;

// Process the analysis result
println!("Table analysis complete: {:?}", result);
```

## Building

To build the WASM module:

```bash
cargo build --target wasm32-wasi --release
```

## Running

To run the WASM module:

```bash
wasmtime target/wasm32-wasi/release/xcell-wasi.wasm
```

## License

MPL-2.0
