# xcell-parser

Parser for XCell type expressions.

## Overview

xcell-parser provides parsing capabilities for XCell type expressions, allowing the project to interpret and process type definitions in a structured manner.

## Features

- Parse XCell type expressions
- Support for complex type structures including Map and Optional types
- Error handling and reporting with line/column information
- Integration with other XCell components

## Usage

```rust
use xcell_parser::{parse_type, TypeExpr};

// Parse a type expression
let type_expr = parse_type("Map<string, i32>").unwrap();

// Process the parsed type
match type_expr {
    TypeExpr::Map { key, value } => println!("Map type with key: {:?}, value: {:?}", key, value),
    TypeExpr::List { element } => println!("List type with element: {:?}", element),
    TypeExpr::Optional { element } => println!("Optional type with element: {:?}", element),
    _ => println!("Other type"),
}
```

## Supported Types

- Primitive types: bool, i8..i64, u8..u64, f32, f64, string, color, time, datetime, date, vec2..vec4
- Collection types: `[T]` (list), `[T; N]` (fixed array), `Vec<T>`, `Map<K, V>`
- Optional type: `T?`
- Reference type: `&TableName`
- Tuple type: `(T1, T2, ...)`
- Named types: custom type names (enums, structs)

## License

MPL-2.0
