# 推进 Oak Fluent 开发 - 产品需求文档

## Overview
- **Summary**: 本项目旨在推进 Oak Fluent 库的开发，特别是实现核心的 FTL (Fluent Translation List) 解析功能，使其能够替代现有的 fluent-bundle 库。
- **Purpose**: 为 iTools 项目提供一个自主可控的国际化解决方案，减少对外部依赖的依赖，提高代码的可维护性和可定制性。
- **Target Users**: 开发人员和维护人员，以及使用 iTools 国际化功能的应用。

## Goals
- 实现 Oak Fluent 的 FTL 解析功能
- 完善 Oak Fluent 的翻译功能
- 确保 Oak Fluent 与现有 API 兼容
- 提供完整的测试覆盖
- 确保 Oak Fluent 能够满足 iTools 的国际化需求

## Non-Goals (Out of Scope)
- 不修改现有的 TranslationProvider trait
- 不修改 iTools 的国际化 API
- 不实现与 Fluent 规范无关的功能
- 不改变现有的文件结构和模块组织

## Background & Context
- 当前 oak-fluent 库的 parse 函数尚未实现，导致无法加载 FTL 文件
- iTools 项目已经移除了对 fluent-bundle 的依赖，全面转向 Oak Fluent
- 为了确保 iTools 的国际化功能正常工作，需要尽快实现 Oak Fluent 的核心功能

## Functional Requirements
- **FR-1**: 实现 FTL 解析功能，支持基本的 FTL 语法
- **FR-2**: 实现消息引用和变量替换功能
- **FR-3**: 实现复数形式和性别形式的支持
- **FR-4**: 完善翻译器功能，确保能够正确翻译带参数的消息
- **FR-5**: 确保与现有 API 兼容，无需修改 iTools 代码

## Non-Functional Requirements
- **NFR-1**: 解析性能不劣于 fluent-bundle
- **NFR-2**: 代码结构清晰，符合 Rust 代码规范
- **NFR-3**: 提供完整的文档和测试
- **NFR-4**: 支持错误处理和错误提示

## Constraints
- **Technical**: 必须与现有的 Oak Fluent API 保持兼容
- **Dependencies**: 依赖 oak-core 库提供的基础功能

## Assumptions
- Oak Core 库提供了必要的解析基础设施
- FTL 语法遵循 Fluent 规范
- 现有的 Oak Fluent 结构设计合理，无需重大修改

## Acceptance Criteria

### AC-1: FTL 解析功能正常
- **Given**: 提供有效的 FTL 字符串
- **When**: 调用 parse 函数
- **Then**: 成功解析并返回 FluentRoot 实例
- **Verification**: `programmatic`

### AC-2: 基本翻译功能正常
- **Given**: 解析后的 FluentRoot 实例
- **When**: 调用翻译器的 translate 方法
- **Then**: 返回正确的翻译结果
- **Verification**: `programmatic`

### AC-3: 带参数的翻译功能正常
- **Given**: 包含变量的 FTL 消息
- **When**: 提供参数并调用翻译方法
- **Then**: 返回替换了变量的翻译结果
- **Verification**: `programmatic`

### AC-4: 复数形式支持
- **Given**: 包含复数形式的 FTL 消息
- **When**: 提供不同的计数参数
- **Then**: 返回正确的复数形式翻译
- **Verification**: `programmatic`

### AC-5: 错误处理正常
- **Given**: 无效的 FTL 语法
- **When**: 调用 parse 函数
- **Then**: 返回适当的错误信息
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要支持所有 Fluent 规范中的高级特性
- [ ] 如何处理复杂的选择表达式
- [ ] 是否需要支持消息引用链
- [ ] 如何优化解析性能