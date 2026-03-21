# XCell RPG Code Generation Fix - Product Requirement Document

## Overview
- **Summary**: Fix the code generation issue in XCell where placeholder files are generated instead of actual code for Unity and Cocos platforms in the RPG examples.
- **Purpose**: Ensure that when running xcell in the RPG example directories, proper TypeScript and C# code is generated instead of placeholder files.
- **Target Users**: XCell users who want to generate code for their game projects.

## Goals
- Fix the code generation process for Unity and Cocos platforms
- Ensure that actual TypeScript and C# code is generated instead of placeholder files
- Verify that the generated code works correctly with the RPG example data

## Non-Goals (Out of Scope)
- Adding new features to the code generation system
- Modifying the existing CSV data files
- Changing the project configuration structure

## Background & Context
The current XCell implementation has code generation stubs that return placeholder files. The actual code generation logic exists in the xcell-generator crate but is not being used by the xcell-analyzer. This results in placeholder files being generated instead of actual code when running xcell in the RPG example directories.

## Functional Requirements
- **FR-1**: The xcell-analyzer should properly integrate with xcell-generator for code generation
- **FR-2**: The write_unity method should generate actual C# code files instead of placeholder files
- **FR-3**: The write_cocos method should generate actual TypeScript code files instead of placeholder files
- **FR-4**: The generated code should correctly process the RPG example data

## Non-Functional Requirements
- **NFR-1**: The code generation process should complete without errors
- **NFR-2**: The generated code should follow the existing code style and conventions
- **NFR-3**: The fix should be minimal and focused on the code generation issue

## Constraints
- **Technical**: Must maintain compatibility with existing project structure and configuration
- **Dependencies**: Must use the existing xcell-generator crate for code generation

## Assumptions
- The xcell-generator crate contains the necessary code generation logic
- The RPG example data files are correctly formatted
- The project configuration files are properly set up

## Acceptance Criteria

### AC-1: Unity Code Generation
- **Given**: The user runs xcell in the rpg-typed directory
- **When**: The code generation process completes
- **Then**: Actual C# files should be generated in the Unity Generated directory instead of placeholder files
- **Verification**: `programmatic`

### AC-2: Cocos Code Generation
- **Given**: The user runs xcell in the rpg-typed directory
- **When**: The code generation process completes
- **Then**: Actual TypeScript files should be generated in the Cocos generated directory instead of placeholder files
- **Verification**: `programmatic`

### AC-3: RPG Untyped Example
- **Given**: The user runs xcell in the rpg-untyped directory
- **When**: The code generation process completes
- **Then**: Actual code files should be generated for both Unity and Cocos platforms
- **Verification**: `programmatic`

### AC-4: Code Generation Success
- **Given**: The user runs the test-rpg.mjs script
- **When**: The script completes execution
- **Then**: No errors should be reported, and all code generation tasks should complete successfully
- **Verification**: `programmatic`

## Open Questions
- [ ] How is the xcell-generator crate supposed to be integrated with xcell-analyzer?
- [ ] Are there any configuration issues that need to be addressed?
- [ ] What specific code generation methods need to be called from xcell-generator?