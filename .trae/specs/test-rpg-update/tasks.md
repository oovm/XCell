# Test RPG Update - The Implementation Plan (Decomposed and Prioritized Task List)

## [x] Task 1: Update test-rpg.mjs to process both rpg-untyped and rpg-typed directories
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - Modify the script to iterate over both rpg-untyped and rpg-typed directories
  - Update the directory paths to reflect the actual directory structure
  - Remove the check for the non-existent `rpg` directory
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-1.1: Script should iterate over both rpg-untyped and rpg-typed directories
  - `programmatic` TR-1.2: Script should not look for a non-existent `rpg` directory
- **Notes**: Ensure the script handles both directories gracefully

## [x] Task 2: Run xcell generate in each directory
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - For each directory (rpg-untyped and rpg-typed), run the xcell generate command
  - Ensure the command is executed with the correct working directory
  - Handle any errors that may occur during execution
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-2.1: xcell generate should run successfully in rpg-untyped directory
  - `programmatic` TR-2.2: xcell generate should run successfully in rpg-typed directory
- **Notes**: Use execSync with stdio: 'inherit' to show output to the user

## [x] Task 3: Verify generated files are created
- **Priority**: P0
- **Depends On**: Task 2
- **Description**:
  - For each directory, check if the generated files are created in the expected locations
  - Check both Unity and Cocos output directories
  - Provide clear feedback to the user about the verification results
- **Acceptance Criteria Addressed**: AC-3
- **Test Requirements**:
  - `programmatic` TR-3.1: Unity generated files should exist in rpg-untyped
  - `programmatic` TR-3.2: Cocos generated files should exist in rpg-untyped
  - `programmatic` TR-3.3: Unity generated files should exist in rpg-typed
  - `programmatic` TR-3.4: Cocos generated files should exist in rpg-typed
- **Notes**: Check the specific paths configured in ProjectSettings.toml

## Notes
- The xcell tool is still in development, and code generation features for Unity and Cocos are not yet implemented
- The write_binary and write_csharp methods in UnityCodegen just return Ok(()) without doing anything
- The write_cocos method in WorkspaceManager is commented out
- The script runs xcell generate successfully, but no files are generated yet