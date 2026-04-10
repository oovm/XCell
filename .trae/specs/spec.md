# XCell 依赖替换 - 产品需求文档

## Overview
- **Summary**: 全面移除 XCell 项目中的 serde_json 和 toml 依赖，替换为 oak-json 和 oak-toml 依赖。
- **Purpose**: 统一使用 oak 系列依赖，减少外部依赖，提高项目的一致性和可维护性。
- **Target Users**: XCell 项目的开发者和维护者。

## Goals
- 移除所有 serde_json 依赖，替换为 oak-json
- 移除所有 toml 依赖，替换为 oak-toml
- 确保项目在替换后能够正常构建和运行
- 保持项目功能不变

## Non-Goals (Out of Scope)
- 不修改项目的核心功能逻辑
- 不添加新的功能
- 不修改项目的外部 API 接口

## Background & Context
- XCell 项目当前使用 serde_json 和 toml 作为 JSON 和 TOML 解析库
- 项目已经引入了 oak-json 和 oak-toml 作为可选依赖
- 为了统一依赖管理，提高项目一致性，需要将所有 serde_json 和 toml 依赖替换为 oak 系列依赖

## Functional Requirements
- **FR-1**: 移除所有 serde_json 依赖，替换为 oak-json
- **FR-2**: 移除所有 toml 依赖，替换为 oak-toml
- **FR-3**: 确保项目在替换后能够正常构建
- **FR-4**: 确保项目在替换后能够正常运行所有测试

## Non-Functional Requirements
- **NFR-1**: 保持项目功能不变
- **NFR-2**: 代码风格和质量保持一致
- **NFR-3**: 构建时间和运行性能不劣化

## Constraints
- **Technical**: 项目使用 Rust 语言，需要确保 oak-json 和 oak-toml 与项目的其他依赖兼容
- **Dependencies**: 依赖于 oak-json 和 oak-toml 的功能完整性

## Assumptions
- oak-json 提供与 serde_json 相同或相似的功能
- oak-toml 提供与 toml 相同或相似的功能
- 替换后不会影响项目的正常运行

## Acceptance Criteria

### AC-1: 依赖替换完成
- **Given**: 项目代码库
- **When**: 执行依赖替换操作
- **Then**: 所有 serde_json 和 toml 依赖被移除，替换为 oak-json 和 oak-toml
- **Verification**: `programmatic`
- **Notes**: 检查所有 Cargo.toml 文件中的依赖项

### AC-2: 项目构建成功
- **Given**: 依赖替换完成
- **When**: 执行 cargo build 命令
- **Then**: 项目成功构建，无编译错误
- **Verification**: `programmatic`

### AC-3: 测试通过
- **Given**: 项目构建成功
- **When**: 执行 cargo test 命令
- **Then**: 所有测试通过
- **Verification**: `programmatic`

### AC-4: 功能保持不变
- **Given**: 依赖替换完成
- **When**: 运行项目的主要功能
- **Then**: 功能与替换前保持一致
- **Verification**: `human-judgment`

## Open Questions
- [ ] oak-json 和 oak-toml 是否完全兼容 serde_json 和 toml 的所有功能？
- [ ] 替换后是否需要修改代码中的使用方式？