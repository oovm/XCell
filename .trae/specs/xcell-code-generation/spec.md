# XCell Code Generation - Product Requirement Document

## Overview
- **Summary**: Implement the missing code generation functionality for XCell, specifically for Unity and Cocos platforms, to generate actual table files instead of placeholders.
- **Purpose**: Enable XCell to generate real code and data files from CSV tables for both Unity and Cocos platforms.
- **Target Users**: Developers working with XCell who need to generate code and data files for their game projects.

## Goals
- Enable Cocos code generation by setting `cocos.enable` to `true` in ProjectSettings.toml
- Implement the missing Cocos JSON data generation functionality
- Fix Unity code generation compilation errors
- Verify that actual table files are generated instead of placeholders

## Non-Goals (Out of Scope)
- Adding new features to XCell beyond code generation
- Modifying the CSV table format
- Changing the overall architecture of XCell

## Background & Context
- XCell is a tool for generating code and data files from CSV tables
- The code generation features for both Unity and Cocos are currently incomplete
- Unity code generation is disabled due to compilation errors
- Cocos code generation has the structure in place but lacks implementation
- ProjectSettings.toml files have `cocos.enable` set to `false`

## Functional Requirements
- **FR-1**: Enable Cocos code generation by setting `cocos.enable` to `true` in ProjectSettings.toml
- **FR-2**: Implement the missing Cocos JSON data generation functionality
- **FR-3**: Fix Unity code generation compilation errors
- **FR-4**: Verify that actual table files are generated instead of placeholders

## Non-Functional Requirements
- **NFR-1**: Code generation should be efficient and produce clean, maintainable code
- **NFR-2**: The generated code should follow best practices for each platform
- **NFR-3**: Error handling should be robust to handle invalid CSV data

## Constraints
- **Technical**: Must work with the existing codebase structure
- **Dependencies**: Requires fixing compilation errors in Unity code generation

## Assumptions
- The CSV tables in both rpg-untyped and rpg-typed directories are valid
- The existing code structure for code generation is correct

## Acceptance Criteria

### AC-1: Cocos code generation is enabled
- **Given**: ProjectSettings.toml files have `cocos.enable` set to `true`
- **When**: xcell generate is run
- **Then**: Cocos code and JSON files should be generated
- **Verification**: `programmatic`

### AC-2: Cocos JSON data generation is implemented
- **Given**: Cocos code generation is enabled
- **When**: xcell generate is run
- **Then**: JSON data files should be generated for each CSV table
- **Verification**: `programmatic`

### AC-3: Unity code generation is fixed
- **Given**: Unity code generation compilation errors are fixed
- **When**: xcell generate is run
- **Then**: Unity C# files should be generated
- **Verification**: `programmatic`

### AC-4: Actual table files are generated
- **Given**: Code generation is implemented
- **When**: xcell generate is run
- **Then**: Actual table files should be generated instead of placeholders
- **Verification**: `programmatic`

## Open Questions
- [ ] What are the specific compilation errors in Unity code generation?
- [ ] How should the JSON data be structured for Cocos?
- [ ] Are there any additional dependencies needed for code generation?