# XCell Code Generation - The Implementation Plan (Decomposed and Prioritized Task List)

## [ ] Task 1: Enable Cocos code generation in ProjectSettings.toml
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - Set `cocos.enable` to `true` in both rpg-untyped and rpg-typed ProjectSettings.toml files
  - Ensure `cocos.json.enable` remains `true`
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-1.1: ProjectSettings.toml files have `cocos.enable` set to `true`
- **Notes**: This is a simple configuration change that enables Cocos code generation

## [ ] Task 2: Implement Cocos JSON data generation
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - Implement the `write_json` method in `CocosCodegen` to generate JSON data files from CSV tables
  - Ensure the JSON structure is appropriate for Cocos
  - Test with the existing CSV tables
- **Acceptance Criteria Addressed**: AC-2, AC-4
- **Test Requirements**:
  - `programmatic` TR-2.1: JSON data files are generated for each CSV table
  - `programmatic` TR-2.2: JSON files contain correct data from the CSV tables
- **Notes**: The JSON data should be structured in a way that's easy to use in Cocos

## [ ] Task 3: Fix Unity code generation compilation errors
- **Priority**: P1
- **Depends On**: None
- **Description**:
  - Identify and fix the compilation errors in Unity code generation
  - Re-enable the commented-out Unity code generation modules
  - Test Unity code generation
- **Acceptance Criteria Addressed**: AC-3, AC-4
- **Test Requirements**:
  - `programmatic` TR-3.1: Unity code generation compiles without errors
  - `programmatic` TR-3.2: Unity C# files are generated for each CSV table
- **Notes**: This may require fixing multiple compilation errors in the Unity code generation modules

## [ ] Task 4: Verify actual table files are generated
- **Priority**: P0
- **Depends On**: Task 2, Task 3
- **Description**:
  - Run xcell generate in both rpg-untyped and rpg-typed directories
  - Verify that actual table files are generated instead of placeholders
  - Check that the generated files contain correct data
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `programmatic` TR-4.1: Actual table files are generated for both Unity and Cocos
  - `programmatic` TR-4.2: Generated files contain correct data from the CSV tables
- **Notes**: This is the final verification step to ensure everything is working correctly