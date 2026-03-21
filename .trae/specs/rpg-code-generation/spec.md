# XCell RPG 代码生成问题分析 - 产品需求文档

## Overview
- **Summary**: 分析 test-rpg.mjs 脚本运行时，在 rpg-typed 目录执行 xcell generate 命令后，未生成 cocos 和 unity 产物的问题
- **Purpose**: 找出代码生成失败的原因并提供解决方案，确保 xcell generate 命令能正确读取 ProjectSettings.toml 配置并生成相应产物
- **Target Users**: XCell 工具的使用者，特别是使用 RPG 示例项目的开发者

## Goals
- 分析 xcell generate 命令执行失败的原因
- 提供修复方案，确保能正确读取 ProjectSettings.toml 配置
- 确保 cocos 和 unity 目录能正确生成产物
- 验证修复后的功能是否正常工作

## Non-Goals (Out of Scope)
- 修改 test-rpg.mjs 脚本的基本逻辑
- 更改 XCell 工具的核心架构
- 扩展 XCell 工具的其他功能

## Background & Context
- XCell 是一个数据表格代码生成工具，支持多种平台的代码生成
- test-rpg.mjs 脚本用于测试 RPG 示例项目的代码生成功能
- rpg-typed 目录包含了 ProjectSettings.toml 配置文件，定义了 cocos 和 unity 两个生成器
- 执行 xcell generate 命令后，应该在对应的目录生成代码产物，但实际上没有生成任何产物

## Functional Requirements
- **FR-1**: xcell generate 命令应能正确读取 ProjectSettings.toml 配置文件
- **FR-2**: 配置文件中的生成器配置应能正确解析和应用
- **FR-3**: 应能根据配置生成 cocos 和 unity 平台的代码产物
- **FR-4**: test-rpg.mjs 脚本应能正确验证生成的产物

## Non-Functional Requirements
- **NFR-1**: 修复方案应保持向后兼容性，不影响其他功能
- **NFR-2**: 修复后应能在 Windows 平台正常运行
- **NFR-3**: 修复方案应简洁明了，易于理解和维护

## Constraints
- **Technical**: 基于现有的 XCell 代码架构，不进行大规模重构
- **Dependencies**: 依赖于现有的 Rust 代码和 Node.js 脚本

## Assumptions
- ProjectSettings.toml 文件的格式是正确的
- xcell 可执行文件已经正确编译
- 测试环境是 Windows 平台

## Acceptance Criteria

### AC-1: ProjectSettings.toml 配置正确解析
- **Given**: rpg-typed 目录存在 ProjectSettings.toml 文件，包含 cocos 和 unity 生成器配置
- **When**: 执行 xcell generate 命令
- **Then**: 命令应能正确读取和解析 ProjectSettings.toml 文件
- **Verification**: `programmatic`

### AC-2: 生成器配置正确应用
- **Given**: ProjectSettings.toml 文件中定义了 cocos 和 unity 生成器
- **When**: 执行 xcell generate 命令
- **Then**: 命令应能正确应用生成器配置，创建对应的产物配置
- **Verification**: `programmatic`

### AC-3: 产物正确生成
- **Given**: 生成器配置正确应用
- **When**: 执行 xcell generate 命令
- **Then**: 应在 cocos 和 unity 目录生成相应的代码产物
- **Verification**: `programmatic`

### AC-4: test-rpg.mjs 脚本验证通过
- **Given**: 产物已正确生成
- **When**: 执行 test-rpg.mjs 脚本
- **Then**: 脚本应能验证产物存在并输出成功信息
- **Verification**: `programmatic`

## Open Questions
- [x] ProjectSettings.toml 文件中的生成器配置是否缺少必要的启用选项？
- [x] xcell generate 命令是否正确处理新格式的生成器配置？
- [x] 生成器配置中的 path 字段是否被正确处理？
- [x] 产物输出目录是否正确创建？