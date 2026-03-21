# Test RPG Update - Product Requirement Document

## Overview
- **Summary**: Update the test-rpg.mjs script to generate code for both rpg-untyped and rpg-typed directories, targeting Unity and Cocos output formats.
- **Purpose**: Ensure that the test script correctly processes both RPG example projects and generates the necessary Unity and Cocos files.
- **Target Users**: Developers working with XCell who need to test the RPG examples.

## Goals
- Update the test-rpg.mjs script to process both rpg-untyped and rpg-typed directories
- Run xcell generate in each directory to generate Unity and Cocos output
- Verify that the generated files are created successfully

## Non-Goals (Out of Scope)
- Modifying the ProjectSettings.toml files
- Changing the xcell executable path or build process
- Adding new features to the xcell tool itself

## Background & Context
- The current test-rpg.mjs script is looking for an `rpg` directory that doesn't exist
- The actual directories are `rpg-untyped` and `rpg-typed`
- Each directory has a ProjectSettings.toml file that configures Unity and Cocos output
- The Unity output is configured to go to `Assets/Scripts/DataTable/Generated`
- The Cocos output is configured to go to `assets/scripts/dataTable/generated`

## Functional Requirements
- **FR-1**: Update the test-rpg.mjs script to process both rpg-untyped and rpg-typed directories
- **FR-2**: Run xcell generate in each directory
- **FR-3**: Verify that the generated files are created successfully

## Non-Functional Requirements
- **NFR-1**: The script should handle errors gracefully
- **NFR-2**: The script should provide clear output about the generation process
- **NFR-3**: The script should be compatible with Windows operating system

## Constraints
- **Technical**: Must use Node.js (mjs) format as specified in the user rules
- **Dependencies**: Requires xcell.exe to be built in the target/debug directory

## Assumptions
- xcell.exe has been built successfully using `cargo build`
- The ProjectSettings.toml files in both rpg-untyped and rpg-typed directories are correctly configured

## Acceptance Criteria

### AC-1: Script processes both rpg-untyped and rpg-typed directories
- **Given**: The test-rpg.mjs script is run
- **When**: The script executes
- **Then**: It should process both rpg-untyped and rpg-typed directories
- **Verification**: `programmatic`

### AC-2: xcell generate runs successfully in both directories
- **Given**: The script processes each directory
- **When**: xcell generate is executed
- **Then**: It should complete without errors
- **Verification**: `programmatic`

### AC-3: Generated files are created
- **Given**: xcell generate completes successfully
- **When**: The script checks for generated files
- **Then**: It should find the generated files in the appropriate locations
- **Verification**: `programmatic`

## Open Questions
- [ ] Should the script create the unity and cocos directories if they don't exist?
- [ ] Should the script check for the existence of the ProjectSettings.toml files before running xcell?