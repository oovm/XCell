# Oak Value and Literal Distinction - Implementation Plan

## [x] Task 1: Implement JsonValue for oak-json
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - Create `JsonValue` enum in `oak-json/src/language/value.rs`
  - Implement variants for all JSON value types (String, Number, Boolean, Array, Object, Null)
  - Add utility methods (as_str, as_f64, as_bool, as_array, as_object, get)
  - Implement Display trait
- **Acceptance Criteria Addressed**: AC-1, AC-5
- **Test Requirements**:
  - `human-judgment` TR-1.1: Verify that `JsonValue` enum is implemented correctly
  - `human-judgment` TR-1.2: Verify that utility methods work as expected
- **Notes**: Follow the pattern established in `TomlValue`

## [x] Task 2: Implement XmlValue for oak-xml
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - Create `XmlValue` enum in `oak-xml/src/language/value.rs`
  - Implement variants for all XML value types
  - Add utility methods similar to `TomlValue`
  - Implement Display trait
- **Acceptance Criteria Addressed**: AC-2, AC-5
- **Test Requirements**:
  - `human-judgment` TR-2.1: Verify that `XmlValue` enum is implemented correctly
  - `human-judgment` TR-2.2: Verify that utility methods work as expected
- **Notes**: Follow the pattern established in `TomlValue`

## [x] Task 3: Implement YamlValue for oak-yaml
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - Create `YamlValue` enum in `oak-yaml/src/language/value.rs`
  - Implement variants for all YAML value types
  - Add utility methods similar to `TomlValue`
  - Implement Display trait
- **Acceptance Criteria Addressed**: AC-3, AC-5
- **Test Requirements**:
  - `human-judgment` TR-3.1: Verify that `YamlValue` enum is implemented correctly
  - `human-judgment` TR-3.2: Verify that utility methods work as expected
- **Notes**: Follow the pattern established in `TomlValue`

## [x] Task 4: Add conversion methods for oak-json
- **Priority**: P1
- **Depends On**: Task 1
- **Description**:
  - Add conversion methods between `JsonValue` and `JsonValueNode`
  - Implement `From` trait for converting between the types
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `human-judgment` TR-4.1: Verify that conversion methods work correctly
- **Notes**: Ensure backward compatibility

## [x] Task 5: Add conversion methods for oak-xml
- **Priority**: P1
- **Depends On**: Task 2
- **Description**:
  - Add conversion methods between `XmlValue` and existing XML AST nodes
  - Implement `From` trait for converting between the types
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `human-judgment` TR-5.1: Verify that conversion methods work correctly
- **Notes**: Ensure backward compatibility

## [x] Task 6: Add conversion methods for oak-yaml
- **Priority**: P1
- **Depends On**: Task 3
- **Description**:
  - Add conversion methods between `YamlValue` and existing YAML AST nodes
  - Implement `From` trait for converting between the types
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `human-judgment` TR-6.1: Verify that conversion methods work correctly
- **Notes**: Ensure backward compatibility

## [x] Task 7: Update documentation for all libraries
- **Priority**: P2
- **Depends On**: Tasks 1-6
- **Description**:
  - Ensure all public structs, enums, methods, and fields have documentation comments
  - Update any existing documentation to reflect the new value types
- **Acceptance Criteria Addressed**: NFR-2
- **Test Requirements**:
  - `human-judgment` TR-7.1: Verify that all public items have documentation
- **Notes**: Follow Rust documentation conventions

## [x] Task 8: Verify implementation consistency
- **Priority**: P2
- **Depends On**: Tasks 1-7
- **Description**:
  - Verify that all three libraries follow the same pattern
  - Ensure consistent naming conventions and method signatures
  - Check for any inconsistencies or errors
- **Acceptance Criteria Addressed**: NFR-1, NFR-3
- **Test Requirements**:
  - `human-judgment` TR-8.1: Verify implementation consistency across libraries
- **Notes**: Compare implementations against `TomlValue` as a reference