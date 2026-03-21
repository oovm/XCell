# UnityStorage Enum Conversion - Product Requirement Document

## Overview
- **Summary**: Convert the UnityStorage struct to an enum that allows only one storage format to be selected at a time, replacing the current struct-based approach that allows multiple storage formats to be enabled simultaneously.
- **Purpose**: Simplify Unity storage configuration by enforcing a single storage format selection, which reduces complexity and potential conflicts in the configuration.
- **Target Users**: Developers using the xcell-config library to configure Unity project storage options.

## Goals
- Convert UnityStorage from a struct to an enum type
- Ensure only one storage format can be selected at a time
- Maintain backward compatibility with existing serialization/deserialization
- Update related code to handle the new enum structure
- Ensure all existing functionality continues to work with the new enum approach

## Non-Goals (Out of Scope)
- Changing the storage format configuration options themselves (binary, JSON, XML, Protobuf)
- Modifying other parts of the Unity configuration system
- Adding new storage formats
- Changing the overall code generation process

## Background & Context
- Currently, UnityStorage is implemented as a struct with fields for each storage format (binary, JSON, XML, Protobuf), all of which can be enabled simultaneously
- This can lead to ambiguity and potential conflicts when multiple storage formats are enabled
- The user wants to enforce a single storage format selection through an enum approach

## Functional Requirements
- **FR-1**: Replace UnityStorage struct with an enum that allows only one storage format to be selected
- **FR-2**: Update serialization and deserialization logic to handle the new enum structure
- **FR-3**: Update UnityCodegen struct to use the new UnityStorage enum
- **FR-4**: Ensure backward compatibility with existing configuration files

## Non-Functional Requirements
- **NFR-1**: Maintain the same level of type safety and error handling
- **NFR-2**: Keep the codebase clean and well-documented
- **NFR-3**: Ensure the change doesn't break existing functionality

## Constraints
- **Technical**: Must maintain compatibility with existing serde serialization/deserialization
- **Dependencies**: Must work with the existing Unity configuration system

## Assumptions
- The current UnityStorage struct is only used within the Unity code generation configuration
- All storage formats (binary, JSON, XML, Protobuf) should remain available as options
- The change should be transparent to users once they update their configuration files

## Acceptance Criteria

### AC-1: UnityStorage is an enum type
- **Given**: The codebase is updated
- **When**: I check the UnityStorage definition
- **Then**: UnityStorage is defined as an enum with variants for each storage format
- **Verification**: `human-judgment`
- **Notes**: The enum should have variants for Binary, Json, Xml, and Protobuf

### AC-2: Only one storage format can be selected
- **Given**: A Unity configuration is created
- **When**: I try to set multiple storage formats
- **Then**: Only one storage format can be selected at a time
- **Verification**: `programmatic`
- **Notes**: This is enforced by the enum type system

### AC-3: Serialization and deserialization work correctly
- **Given**: The new UnityStorage enum is implemented
- **When**: I serialize and deserialize Unity configuration
- **Then**: The storage format is correctly preserved
- **Verification**: `programmatic`
- **Notes**: Should test both new and old format compatibility

### AC-4: UnityCodegen uses the new enum
- **Given**: The codebase is updated
- **When**: I check the UnityCodegen struct
- **Then**: It uses the new UnityStorage enum instead of the old struct
- **Verification**: `human-judgment`

### AC-5: Existing functionality works
- **Given**: The change is implemented
- **When**: I run existing code that uses Unity configuration
- **Then**: It works the same as before (with appropriate configuration changes)
- **Verification**: `programmatic`

## Open Questions
- [ ] How should the default storage format be handled?
- [ ] What migration path should be provided for existing configurations?