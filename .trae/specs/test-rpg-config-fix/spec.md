# XCell RPG 配置读取问题分析 - 产品需求文档

## 概述
- **Summary**: 分析并修复 `test-rpg.mjs` 运行错误以及 `xcell generate` 命令无法生成产物的问题。
- **Purpose**: 解决 XCell 在 RPG 示例项目中无法正确读取配置文件和生成代码的问题。
- **Target Users**: XCell 工具的使用者，特别是使用 RPG 示例项目的开发者。

## Goals
- 分析 `test-rpg.mjs` 运行错误的原因
- 修复 `xcell generate` 命令无法生成 Cocos 和 Unity 产物的问题
- 确保 `ProjectSettings.toml` 配置文件能被正确读取和解析
- 验证修复后能够成功生成 Cocos 和 Unity 产物

## Non-Goals (Out of Scope)
- 不修改 XCell 的核心架构
- 不添加新功能，只修复现有问题
- 不修改示例项目的业务逻辑

## Background & Context
- `test-rpg.mjs` 脚本用于测试 XCell 在 RPG 示例项目中的功能
- `xcell generate` 命令应该根据 `ProjectSettings.toml` 配置生成 Cocos 和 Unity 产物
- 目前运行 `test-rpg.mjs` 时，xcell 命令执行成功，但没有生成任何产物
- 配置文件 `ProjectSettings.toml` 中定义了两个生成器：Cocos 和 Unity

## Functional Requirements
- **FR-1**: `xcell generate` 命令应正确读取 `ProjectSettings.toml` 配置文件
- **FR-2**: `xcell generate` 命令应根据配置生成 Cocos 产物
- **FR-3**: `xcell generate` 命令应根据配置生成 Unity 产物
- **FR-4**: `test-rpg.mjs` 脚本应能成功验证生成的产物

## Non-Functional Requirements
- **NFR-1**: 修复应保持向后兼容性，不破坏现有功能
- **NFR-2**: 修复应简洁明了，不引入不必要的复杂性
- **NFR-3**: 修复后应能在 Windows 环境下正常运行

## Constraints
- **Technical**: 项目使用 Rust 语言开发，配置文件使用 TOML 格式
- **Dependencies**: 依赖 serde 库进行配置文件的序列化和反序列化

## Assumptions
- `ProjectSettings.toml` 配置文件格式正确
- XCell 可执行文件已正确编译
- 示例项目文件结构完整

## Acceptance Criteria

### AC-1: 配置文件读取
- **Given**: 存在有效的 `ProjectSettings.toml` 配置文件
- **When**: 运行 `xcell generate` 命令
- **Then**: 命令应成功读取配置文件并解析生成器配置
- **Verification**: `programmatic`

### AC-2: Cocos 产物生成
- **Given**: 配置文件中启用了 Cocos 生成器
- **When**: 运行 `xcell generate` 命令
- **Then**: 应在指定的输出目录生成 Cocos 产物
- **Verification**: `programmatic`

### AC-3: Unity 产物生成
- **Given**: 配置文件中启用了 Unity 生成器
- **When**: 运行 `xcell generate` 命令
- **Then**: 应在指定的输出目录生成 Unity 产物
- **Verification**: `programmatic`

### AC-4: 脚本验证
- **Given**: 运行 `test-rpg.mjs` 脚本
- **When**: 脚本执行完成
- **Then**: 脚本应成功验证生成的 Cocos 和 Unity 产物
- **Verification**: `programmatic`

## Open Questions
- [ ] 配置文件解析是否存在问题？
- [ ] 生成器配置是否正确传递给代码生成模块？
- [ ] 代码生成模块是否正确处理生成请求？