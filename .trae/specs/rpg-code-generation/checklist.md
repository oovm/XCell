# XCell RPG Code Generation Fix - Verification Checklist

## Code Generation Fix Verification
- [ ] xcell-generator is properly added as a dependency to xcell-analyzer
- [ ] write_unity method is updated to use actual code generation logic
- [ ] write_cocos method is updated to use actual code generation logic
- [ ] No compilation errors after the changes

## RPG-Typed Example Verification
- [ ] Unity Generated directory contains actual C# files (not placeholders)
- [ ] Cocos generated directory contains actual TypeScript files (not placeholders)
- [ ] All RPG data tables are properly processed
- [ ] No Placeholder.ts or Placeholder.cs files exist

## RPG-Untyped Example Verification
- [ ] Unity Generated directory contains actual C# files (not placeholders)
- [ ] Cocos generated directory contains actual TypeScript files (not placeholders)
- [ ] All RPG data tables are properly processed
- [ ] No Placeholder.ts or Placeholder.cs files exist

## End-to-End Test Verification
- [ ] test-rpg.mjs script runs without errors
- [ ] All code generation tasks complete successfully
- [ ] No placeholder files are generated in either example
- [ ] Generated code follows existing code style and conventions