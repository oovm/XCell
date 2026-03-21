# Test RPG Script Error Analysis - Product Requirement Document

## Overview
- **Summary**: Analysis and resolution of test-rpg.mjs script execution errors where no output files are generated in cocos and unity directories despite proper ProjectSetting.toml configuration.
- **Purpose**: Identify why xcell generate command fails to produce expected output files in cocos and unity directories when executed through test-rpg.mjs script.
- **Target Users**: XCell development team and users working with RPG examples

## Goals
- Identify the root cause of test-rpg.mjs script execution errors
- Ensure xcell generate correctly reads ProjectSetting.toml from rpg-typed directory
- Verify cocos and unity output directories are properly generated
- Provide a fix for the script execution issue

## Non-Goals (Out of Scope)
- Modifying xcell core functionality
- Changing ProjectSetting.toml configuration format
- Adding new features to test-rpg.mjs script

## Background & Context
- The test-rpg.mjs script is designed to execute xcell generate command on RPG example projects
- ProjectSetting.toml in rpg-typed directory is configured to generate output for both Cocos and Unity targets
- Despite correct configuration, no output files are being generated

## Functional Requirements
- **FR-1**: test-rpg.mjs script should successfully execute xcell generate command
- **FR-2**: xcell generate should read ProjectSetting.toml from the correct directory
- **FR-3**: xcell generate should generate output files in both cocos and unity directories

## Non-Functional Requirements
- **NFR-1**: Script execution should provide clear error messages when failures occur
- **NFR-2**: Output directories should be created if they don't exist
- **NFR-3**: Generated files should match expected structure and content

## Constraints
- **Technical**: Windows operating system, xcell executable must be present in target/debug
- **Dependencies**: Rust build system (cargo build required to generate xcell.exe)

## Assumptions
- xcell.exe is properly built and functional
- ProjectSetting.toml configuration is correct
- CSV files in rpg-typed directory are valid

## Acceptance Criteria

### AC-1: Script Execution
- **Given**: xcell.exe exists in target/debug directory
- **When**: test-rpg.mjs script is executed
- **Then**: Script runs without errors