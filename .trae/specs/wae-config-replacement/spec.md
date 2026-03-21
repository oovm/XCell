# WAE Config Replacement - Product Requirement Document

## Overview
- **Summary**: Replace the figment-based implementation in wae-config with a new implementation based on itools-config, which offers better performance and similar functionality.
- **Purpose**: Improve the performance of WAE's configuration management by leveraging the efficient itools-config library.
- **Target Users**: Developers working with WAE who need efficient configuration management.

## Goals
- Replace figment dependency with itools-config in wae-config
- Maintain the same API and functionality as the original figment-based implementation
- Ensure backward compatibility with existing code
- Verify that the new implementation performs as well as itools-config

## Non-Goals (Out of Scope)
- Adding new features to wae-config beyond what was already implemented
- Modifying the API of wae-config
- Changing the way configuration is used in other parts of WAE

## Background & Context
- wae-config currently uses figment for configuration management
- itools-config is a high-performance configuration library with similar functionality
- The goal is to leverage itools-config's performance while maintaining the same API

## Functional Requirements
- **FR-1**: Replace figment dependency with itools-config in wae-config's Cargo.toml
- **FR-2**: Implement the same ConfigLoader API using itools-config
- **FR-3**: Maintain support for loading config from TOML, YAML, and environment variables
- **FR-4**: Maintain support for merging default values
- **FR-5**: Maintain the same error handling behavior

## Non-Functional Requirements
- **NFR-1**: The new implementation should be at least as performant as itools-config
- **NFR-2**: The new implementation should maintain backward compatibility
- **NFR-3**: The code should follow the same style and conventions as the original

## Constraints
- **Technical**: Must maintain the same API as the original figment-based implementation
- **Dependencies**: Must replace figment with itools-config

## Assumptions
- itools-config provides all the necessary functionality to replace figment
- The existing API of wae-config is well-designed and should be preserved

## Acceptance Criteria

### AC-1: figment dependency is replaced with itools-config
- **Given**: wae-config's Cargo.toml is updated
- **When**: The project is built
- **Then**: It should build successfully without figment dependency
- **Verification**: `programmatic`

### AC-2: ConfigLoader API is preserved
- **Given**: The new implementation is complete
- **When**: Existing code uses ConfigLoader
- **Then**: It should work exactly as before
- **Verification**: `programmatic`

### AC-3: All configuration sources are supported
- **Given**: Config is loaded from TOML, YAML, and environment variables
- **When**: The new implementation is used
- **Then**: All sources should be loaded correctly
- **Verification**: `programmatic`

### AC-4: Default values are merged correctly
- **Given**: Default values are provided
- **When**: The new implementation is used
- **Then**: Default values should be merged correctly
- **Verification**: `programmatic`

### AC-5: Error handling is preserved
- **Given**: Invalid configuration is provided
- **When**: The new implementation is used
- **Then**: It should return the same error types as before
- **Verification**: `programmatic`

## Open Questions
- [ ] What are the exact performance improvements of itools-config over figment?
- [ ] Are there any edge cases in the original implementation that need special handling?
- [ ] How to ensure backward compatibility with existing code?