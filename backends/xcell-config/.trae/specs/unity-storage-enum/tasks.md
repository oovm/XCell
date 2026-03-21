# UnityStorage Enum Conversion - Implementation Plan

## [x] Task 1: Define the UnityStorage enum
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - Replace the UnityStorage struct with an enum type
  - Define enum variants for each storage format (Binary, Json, Xml, Protobuf)
  - Each variant should contain the corresponding configuration struct
- **Acceptance Criteria Addressed**: AC-1, AC-2
- **Test Requirements**:
  - `human-judgment` TR-1.1: Verify UnityStorage is defined as an enum with appropriate variants
  - `programmatic` TR-1.2: Test that only one storage format can be selected at a time
- **Notes**: Ensure each enum variant contains the corresponding configuration struct (UnityBinaryConfig, UnityJsonConfig, etc.)

## [x] Task 2: Update UnityCodegen struct to use the new enum
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - Update the UnityCodegen struct's storage field to use the new UnityStorage enum
  - Update any related code that accesses the storage field
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `human-judgment` TR-2.1: Verify UnityCodegen uses the new UnityStorage enum
  - `programmatic` TR-2.2: Test that UnityCodegen can be instantiated with the new enum
- **Notes**: Check all places where UnityCodegen.storage is accessed and update them accordingly

## [/] Task 3: Update serialization logic for the new enum
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - Update the serialization implementation in ser.rs to handle the new enum structure
  - Ensure the enum is properly serialized to the appropriate format
- **Acceptance Criteria Addressed**: AC-3
- **Test Requirements**:
  - `programmatic` TR-3.1: Test serialization of the new UnityStorage enum
  - `programmatic` TR-3.2: Verify serialized output matches expected format
- **Notes**: Use serde's enum serialization features

## [ ] Task 4: Update deserialization logic for the new enum
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - Update the deserialization implementation in der.rs to handle the new enum structure
  - Ensure backward compatibility with existing configuration formats
- **Acceptance Criteria Addressed**: AC-3, AC-4
- **Test Requirements**:
  - `programmatic` TR-4.1: Test deserialization of the new UnityStorage enum format
  - `programmatic` TR-4.2: Test deserialization of existing configuration format (backward compatibility)
- **Notes**: Need to handle both the new enum format and the old struct format for backward compatibility

## [ ] Task 5: Update UnityCodegenHelper for backward compatibility
- **Priority**: P1
- **Depends On**: Task 4
- **Description**:
  - Update the UnityCodegenHelper struct in der.rs to handle both old and new storage formats
  - Ensure existing configuration files can still be deserialized
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `programmatic` TR-5.1: Test deserialization of old format configuration files
  - `programmatic` TR-5.2: Test deserialization of new format configuration files
- **Notes**: This is crucial for ensuring existing projects don't break

## [ ] Task 6: Test the implementation
- **Priority**: P0
- **Depends On**: Tasks 1-5
- **Description**:
  - Run existing tests to ensure nothing is broken
  - Create new tests for the new enum functionality
  - Test both serialization and deserialization with different storage formats
- **Acceptance Criteria Addressed**: AC-5
- **Test Requirements**:
  - `programmatic` TR-6.1: Run existing test suite
  - `programmatic` TR-6.2: Test all storage format enum variants
  - `programmatic` TR-6.3: Test edge cases and error handling
- **Notes**: Make sure all existing functionality still works as expected