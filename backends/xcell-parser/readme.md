# xcell-parser

Parser for XCell type expressions using parser combinators.

## Overview

xcell-parser provides parsing capabilities for XCell type expressions, allowing the project to interpret and process type definitions in a structured manner.

## Features

- Parse XCell type expressions
- Support for complex type structures
- Error handling and reporting
- Integration with other XCell components

## Usage

```rust
use xcell_parser::parse_type_expression;

// Parse a type expression
let type_expr = parse_type_expression("List<Map<String, Int>>")?;

// Process the parsed type
match type_expr {
    TypeExpression::List(inner) => println!("List type with inner type: {:?}", inner),
    TypeExpression::Map(key, value) => println!("Map type with key: {:?}, value: {:?}", key, value),
    // Handle other types...
}
```

## Supported Types

- Primitive types: Int, Float, String, Boolean, Date
- Collection types: List, Map
- Custom types: User-defined types

## License

MPL-2.0
