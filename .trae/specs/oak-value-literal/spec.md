# Oak Value and Literal Distinction - Product Requirement Document

## Overview
- **Summary**: Implement value and literal distinction for oak-json, oak-xml, and oak-yaml libraries, following the pattern established in oak-toml.
- **Purpose**: To provide a consistent way to represent pure values (without source location) and literal nodes (with source location) across all Oak format libraries.
- **Target Users**: Developers using the Oak libraries for parsing and processing JSON, XML, YAML, and TOML files.

## Goals
- Implement `JsonValue` enum for oak-json that represents pure JSON values without source location
- Implement `XmlValue` enum for oak-xml that represents pure XML values without source location
- Implement `YamlValue` enum for oak-yaml that represents pure YAML values without source location
- Ensure all value enums follow the same pattern as `TomlValue` in oak-toml
- Maintain backward compatibility with existing AST structures

## Non-Goals (Out of Scope)
- Modifying the existing AST node structures (e.g., `JsonValueNode`)
- Changing the parsing logic of any of the libraries
- Adding new features beyond the value/literal distinction

## Background & Context
- oak-toml already implements a clear distinction between `TomlValue` (pure values) and `TomlValueNode` (literal nodes with source location)
- This distinction provides a clean way to work with values independently of their source code representation
- The other Oak libraries (oak-json, oak-xml, oak-yaml) currently only have literal node representations

## Functional Requirements
- **FR-1**: Create `JsonValue` enum in oak-json that represents pure JSON values
- **FR-2**: Create `XmlValue` enum in oak-xml that represents pure XML values
- **FR-3**: Create `YamlValue` enum in oak-yaml that represents pure YAML values
- **FR-4**: Implement conversion methods between value enums and existing AST node structures
- **FR-5**: Provide utility methods for working with the value enums (e.g., `as_str()`, `as_integer()`, etc.)

## Non-Functional Requirements
- **NFR-1**: The implementation should follow the same pattern as oak-toml's `TomlValue`
- **NFR-2**: All public structs, enums, methods, and fields must have documentation comments
- **NFR-3**: The implementation should be consistent across all three libraries
- **NFR-4**: No breaking changes to existing APIs

## Constraints
- **Technical**: Must use Rust programming language
- **Dependencies**: Must work with existing Oak core libraries
- **Timeline**: Implementation should be completed as a single task

## Assumptions
- The existing AST structures for oak-json, oak-xml, and oak-yaml are already in place
- The pattern established in oak-toml is the correct approach to follow
- All libraries use similar structures for representing values

## Acceptance Criteria

### AC-1: JsonValue Implementation
- **Given**: oak-json library
- **When**: I implement the `JsonValue` enum
- **Then**: It should represent all JSON value types without source location information
- **Verification**: `human-judgment`
- **Notes**: Should follow the same pattern as `TomlValue`

### AC-2: XmlValue Implementation
- **Given**: oak-xml library
- **When**: I implement the `XmlValue` enum
- **Then**: It should represent all XML value types without source location information
- **Verification**: `human-judgment`
- **Notes**: Should follow the same pattern as `TomlValue`

### AC-3: YamlValue Implementation
- **Given**: oak-yaml library
- **When**: I implement the `YamlValue` enum
- **Then**: It should represent all YAML value types without source location information
- **Verification**: `human-judgment`
- **Notes**: Should follow the same pattern as `TomlValue`

### AC-4: Conversion Methods
- **Given**: Implemented value enums
- **When**: I add conversion methods between value enums and AST nodes
- **Then**: I should be able to convert between pure values and literal nodes
- **Verification**: `human-judgment`

### AC-5: Utility Methods
- **Given**: Implemented value enums
- **When**: I add utility methods (e.g., `as_str()`, `as_integer()`)
- **Then**: These methods should work consistently across all value enums
- **Verification**: `human-judgment`

## Open Questions
- [ ] What specific value types need to be represented for each format (JSON, XML, YAML)?
- [ ] Are there any format-specific considerations for the value enums?