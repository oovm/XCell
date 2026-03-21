# XCell 代码生成问题修复 - 实现计划

## [x] 任务 1: 实现 UnityCodegen::write_csharp() 方法
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 实现 UnityCodegen::write_csharp() 方法，生成实际的 C# 代码文件
  - 确保生成的代码包含正确的类定义和字段
  - 遵循现有的代码结构和模式
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-1.1: 运行 xcell 命令后，Unity 目录中应生成实际的 C# 代码文件，而非 Placeholder.cs
  - `programmatic` TR-1.2: 生成的 C# 代码文件应包含正确的类定义和字段
- **Notes**: 需要参考现有的 Cocos 代码生成实现，确保逻辑一致

## [x] 任务 2: 完善 Cocos 代码生成逻辑
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 完善 Cocos 代码生成逻辑，确保生成实际的 TypeScript 代码文件
  - 确保生成的代码包含正确的类定义和字段
  - 确保 JSON 数据文件也能正确生成
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-2.1: 运行 xcell 命令后，Cocos 目录中应生成实际的 TypeScript 代码文件，而非 Placeholder.ts
  - `programmatic` TR-2.2: 生成的 TypeScript 代码文件应包含正确的类定义和字段
  - `programmatic` TR-2.3: 生成的 JSON 数据文件应包含正确的数据结构
- **Notes**: 现有代码已经有部分实现，需要完善并确保正确生成文件

## [/] 任务 3: 构建项目并验证修复
- **Priority**: P0
- **Depends On**: 任务 1, 任务 2
- **Description**: 
  - 构建 XCell 项目，确保代码编译通过
  - 运行 test-rpg.mjs 脚本，验证代码生成功能
  - 检查生成的文件是否正确，包含实际的代码而非占位符
- **Acceptance Criteria Addressed**: AC-3
- **Test Requirements**:
  - `programmatic` TR-3.1: 项目构建成功，无编译错误
  - `programmatic` TR-3.2: test-rpg.mjs 脚本运行成功，无错误
  - `programmatic` TR-3.3: 生成的文件应是实际的代码文件，包含正确的内容
- **Notes**: 确保测试环境配置正确，xcell 可执行文件路径正确