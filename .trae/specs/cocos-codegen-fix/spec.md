# Cocos Codegen Fix - Product Requirement Document

## Overview
- **Summary**: Fix the Cocos code generation to properly process CSV files and generate TypeScript files with appropriate names based on the CSV file names.
- **Purpose**: The current implementation generates random test files with timestamps instead of processing actual CSV files, which is not what users expect.
- **Target Users**: Developers using XCell to generate Cocos TypeScript code from CSV files.

## Goals
- Fix the Cocos code generation to process actual CSV files
- Generate TypeScript files with appropriate names based on CSV file names (e.g., Item.csv → ItemTable.ts)
- Ensure the generated TypeScript classes have the correct fields based on the CSV headers
- Maintain compatibility with existing project structure

## Non-Goals (Out of Scope)
- Modifying the CSV file format
- Changing the Unity code generation logic
- Adding new features to the code generation

## Background & Context
The current CocosCodegen implementation in xcell-generator has a simplified write_typescript method that generates test files with timestamps instead of processing actual CSV files. This is causing confusion for users who expect the generated files to match their CSV file names.

## Functional Requirements
- **FR-1**: The Cocos code generator should process all CSV files in the project directory
- **FR-2**: For each CSV file, generate a TypeScript file with the name {CSVName}Table.ts
- **FR-3**: The generated TypeScript class should have fields corresponding to the CSV headers
- **FR-4**: The generated TypeScript class should be properly structured with appropriate types

## Non-Functional Requirements
- **NFR-1**: The code generation should be efficient and run in a reasonable time
- **NFR-2**: The generated code should be clean and follow TypeScript best practices
- **NFR-3**: The fix should be backward compatible with existing projects

## Constraints
- **Technical**: The fix should work with the existing XCell architecture
- **Dependencies**: The fix depends on the workspace manager providing access to CSV data

## Assumptions
- The CSV files are properly formatted with headers
- The workspace manager correctly loads and processes the CSV files
- The Cocos project structure follows the standard layout

## Acceptance Criteria

### AC-1: Generate correct TypeScript files from CSV files
- **Given**: A project with CSV files (Item.csv, Monsters.csv, etc.)
- **When**: Running xcell generate
- **Then**: TypeScript files are generated with names like ItemTable.ts, MonstersTable.ts, etc.
- **Verification**: `programmatic`

### AC-2: Generated TypeScript classes have correct fields
- **Given**: A CSV file with headers (id, name, type, etc.)
- **When**: Running xcell generate
- **Then**: The generated TypeScript class has fields corresponding to the CSV headers
- **Verification**: `programmatic`

### AC-3: Generated TypeScript files are properly structured
- **Given**: A valid CSV file
- **When**: Running xcell generate
- **Then**: The generated TypeScript file has a properly structured class with appropriate types
- **Verification**: `human-judgment`

## Open Questions
- [ ] How to handle different CSV file types (class, dictionary, list, etc.)
- [ ] What types to use for different CSV column types
- [ ] How to handle nested structures or complex types