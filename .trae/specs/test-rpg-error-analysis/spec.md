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