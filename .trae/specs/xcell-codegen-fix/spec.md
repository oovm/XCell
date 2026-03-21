# XCell 代码生成问题修复 - 产品需求文档

## 概述
- **Summary**: 修复 XCell 在 RPG 示例目录中生成占位符文件而非实际代码的问题，确保 Unity 和 Cocos 平台能正确生成代码文件。
- **Purpose**: 解决用户在运行 xcell 命令时，生成的文件是 Placeholder.ts 和 Placeholder.cs 而非实际代码的问题，确保代码生成功能正常工作。
- **Target Users**: XCell 工具的用户，特别是使用 Unity 和 Cocos 平台的开发者。

## 目标
- 修复 Unity C# 代码生成，确保生成实际的 C# 代码文件而非占位符
- 修复 Cocos TypeScript 代码生成，确保生成实际的 TypeScript 代码文件而非占位符
- 确保生成的代码文件包含正确的类定义和字段
- 验证修复后的代码生成功能在 RPG 示例目录中正常工作

## 非目标（超出范围）
- 不修改代码生成的整体架构
- 不添加新的代码生成功能
- 不修改现有的配置文件格式

## 背景与上下文
- XCell 是一个代码生成工具，用于为 Unity 和 Cocos 等平台生成数据驱动的代码
- 目前在运行 `test-rpg.mjs` 脚本时，生成的文件是 Placeholder.ts 和 Placeholder.cs，而非实际的代码文件
- 问题出在 UnityCodegen::write_csharp() 和 Cocos 代码生成的实现上，这些方法目前是空实现或不完整的实现

## 功能需求
- **FR-1**: 实现 UnityCodegen::write_csharp() 方法，生成实际的 C# 代码文件
- **FR-2**: 完善 Cocos 代码生成逻辑，确保生成实际的 TypeScript 代码文件
- **FR-3**: 确保生成的代码文件包含正确的类定义和字段
- **FR-4**: 验证修复后的代码生成功能在 RPG 示例目录中正常工作

## 非功能需求
- **NFR-1**: 代码生成性能应保持高效，不增加明显的执行时间
- **NFR-2**: 生成的代码应符合 Unity 和 Cocos 的代码规范
- **NFR-3**: 修复应保持向后兼容性，不破坏现有的功能

## 约束
- **Technical**: 使用 Rust 语言实现，遵循现有的代码结构和模式
- **Business**: 修复应在合理的时间内完成，不影响其他功能的开发
- **Dependencies**: 依赖现有的 xcell-analyzer、xcell-config 等模块

## 假设
- 现有的配置文件格式正确，不需要修改
- 现有的表格数据结构正确，能够被正确解析
- 修复后的代码生成功能应与现有的测试脚本兼容

## 验收标准

### AC-1: Unity C# 代码生成
- **Given**: 运行 xcell 命令在 RPG 示例目录
- **When**: 生成 Unity 代码文件
- **Then**: 生成的文件应是实际的 C# 代码文件，包含正确的类定义和字段，而非 Placeholder.cs
- **Verification**: `programmatic`

### AC-2: Cocos TypeScript 代码生成
- **Given**: 运行 xcell 命令在 RPG 示例目录
- **When**: 生成 Cocos 代码文件
- **Then**: 生成的文件应是实际的 TypeScript 代码文件，包含正确的类定义和字段，而非 Placeholder.ts
- **Verification**: `programmatic`

### AC-3: 测试脚本运行
- **Given**: 运行 test-rpg.mjs 脚本
- **When**: 执行 xcell 命令并验证生成结果
- **Then**: 脚本应成功运行，验证生成的文件存在且内容正确
- **Verification**: `programmatic`

## 未解决的问题
- [ ] 是否需要修改现有的代码生成架构以避免循环依赖问题
- [ ] 是否需要添加更多的测试用例来验证代码生成功能