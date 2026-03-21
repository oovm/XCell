# XCell RPG Code Generation Fix - Implementation Plan

## [ ] Task 1: Add xcell-generator dependency to xcell-analyzer
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - Add xcell-generator as a dependency in xcell-analyzer's Cargo.toml
  - Ensure the dependency is properly configured
- **Acceptance Criteria Addressed**: AC-1, AC-2, AC-3, AC-4
- **Test Requirements**:
  - `programmatic` TR-1.1: xcell-analyzer compiles successfully with xcell-generator dependency
  - `programmatic` TR-1.2: No compilation errors related to the new dependency
- **Notes**: This is a prerequisite for integrating the code generation logic

## [ ] Task 2: Update write_unity method to use xcell-generator
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - Modify the write_unity method in WorkspaceManager to use the actual code generation logic from xcell-generator
  - Replace the placeholder implementation with a call to the appropriate Unity code generation method
- **Acceptance Criteria Addressed**: AC-1, AC-3, AC-4
- **Test Requirements**:
  - `programmatic` TR-2.1: Unity code generation completes without errors
  - `programmatic` TR-2.2: Actual C# files are generated instead of placeholder files
- **Notes**: Need to ensure the Unity code generation module is properly enabled

## [ ] Task 3: Update write_cocos method to use xcell-generator
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - Modify the write_cocos method in WorkspaceManager to use the actual code generation logic from xcell-generator
  - Replace the placeholder implementation with a call to the appropriate Cocos code generation method
- **Acceptance Criteria Addressed**: AC-2, AC-3, AC-4
- **Test Requirements**:
  - `programmatic` TR-3.1: Cocos code generation completes without errors
  - `programmatic` TR-3.2: Actual TypeScript files are generated instead of placeholder files
- **Notes**: The Cocos code generation module appears to be more complete than Unity

## [ ] Task 4: Test code generation with rpg-typed example
- **Priority**: P1
- **Depends On**: Task 2, Task 3
- **Description**:
  - Run xcell in the rpg-typed directory
  - Verify that actual code files are generated for both Unity and Cocos platforms
  - Check that no placeholder files are present
- **Acceptance Criteria Addressed**: AC-1, AC-2, AC-4
- **Test Requirements**:
  - `programmatic` TR-4.1: Unity Generated directory contains actual C# files
  - `programmatic` TR-4.2: Cocos generated directory contains actual TypeScript files
  - `programmatic` TR-4.3: No Placeholder.ts or Placeholder.cs files exist
- **Notes**: This tests the core functionality of the fix

## [ ] Task 5: Test code generation with rpg-untyped example
- **Priority**: P1
- **Depends On**: Task 2, Task 3
- **Description**:
  - Run xcell in the rpg-untyped directory
  - Verify that actual code files are generated for both Unity and Cocos platforms
  - Check that no placeholder files are present
- **Acceptance Criteria Addressed**: AC-3, AC-4
- **Test Requirements**:
  - `programmatic` TR-5.1: Unity Generated directory contains actual C# files
  - `programmatic` TR-5.2: Cocos generated directory contains actual TypeScript files
  - `programmatic` TR-5.3: No Placeholder.ts or Placeholder.cs files exist
- **Notes**: This ensures the fix works for both typed and untyped examples

## [ ] Task 6: Run the test-rpg.mjs script
- **Priority**: P1
- **Depends On**: Task 4, Task 5
- **Description**:
  - Execute the test-rpg.mjs script to test the entire code generation process
  - Verify that no errors are reported
  - Confirm that all code generation tasks complete successfully
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `programmatic` TR-6.1: The script runs without errors
  - `programmatic` TR-6.2: All code generation tasks report success
  - `programmatic` TR-6.3: No placeholder files are generated
- **Notes**: This tests the end-to-end functionality as used by users