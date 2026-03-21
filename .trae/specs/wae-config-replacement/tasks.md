# WAE Config Replacement - The Implementation Plan (Decomposed and Prioritized Task List)

## [ ] Task 1: Update wae-config Cargo.toml to replace figment with itools-config
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - Remove figment dependency from Cargo.toml
  - Add itools-config dependency
  - Ensure all necessary features are enabled
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-1.1: Cargo.toml no longer has figment dependency
  - `programmatic` TR-1.2: Cargo.toml has itools-config dependency with appropriate features
- **Notes**: Ensure the itools-config version is compatible with the existing codebase

## [ ] Task 2: Implement ConfigLoader using itools-config
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - Rewrite the ConfigLoader struct to use itools-config instead of figment
  - Maintain the same API as the original implementation
  - Implement all methods: new, with_toml, with_yaml, with_env, with_env_separator, with_defaults, extract, extract_with_context
- **Acceptance Criteria Addressed**: AC-2, AC-3, AC-4, AC-5
- **Test Requirements**:
  - `programmatic` TR-2.1: ConfigLoader API is identical to the original
  - `programmatic` TR-2.2: All methods work as expected
  - `programmatic` TR-2.3: Error handling is preserved
- **Notes**: Pay attention to the details of how figment handles configuration merging and ensure itools-config does the same

## [ ] Task 3: Update load_config and from_env functions
- **Priority**: P0
- **Depends On**: Task 2
- **Description**:
  - Update the load_config function to use the new ConfigLoader implementation
  - Update the from_env function to use the new ConfigLoader implementation
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-3.1: load_config function works as before
  - `programmatic` TR-3.2: from_env function works as before
- **Notes**: Ensure these functions maintain the same signature and behavior

## [ ] Task 4: Test the new implementation
- **Priority**: P0
- **Depends On**: Task 3
- **Description**:
  - Run existing tests to ensure they pass
  - Test loading config from different sources
  - Test merging default values
  - Test error handling
- **Acceptance Criteria Addressed**: AC-2, AC-3, AC-4, AC-5
- **Test Requirements**:
  - `programmatic` TR-4.1: All existing tests pass
  - `programmatic` TR-4.2: Config loads correctly from all sources
  - `programmatic` TR-4.3: Default values are merged correctly
  - `programmatic` TR-4.4: Error handling works as expected
- **Notes**: Create additional tests if needed to ensure coverage

## [ ] Task 5: Verify performance
- **Priority**: P1
- **Depends On**: Task 4
- **Description**:
  - Compare the performance of the new implementation with the original
  - Ensure the new implementation is at least as performant as itools-config
- **Acceptance Criteria Addressed**: NFR-1
- **Test Requirements**:
  - `programmatic` TR-5.1: Performance is at least as good as the original
  - `programmatic` TR-5.2: Performance is comparable to itools-config
- **Notes**: Use benchmarking tools to measure performance