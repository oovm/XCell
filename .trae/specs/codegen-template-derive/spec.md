# XCell 代码生成规范 - 使用 derive Template

## 概述
- **摘要**：本规范定义了 XCell 项目中代码生成的标准方法，要求所有代码生成必须使用 dejavu-macros 的 `#[derive(Template)]` 宏，禁止使用手工字符串拼接。
- **目的**：提高代码生成的可维护性、可读性和可靠性，避免手工字符串拼接带来的错误和维护困难。
- **目标用户**：XCell 项目的开发者和维护者。

## 目标
- 所有代码生成模块必须使用 `#[derive(Template)]` 宏
- 禁止使用手工字符串拼接进行代码生成
- 确保 dejavu-engine 提供足够的特性支持 XCell 的代码生成需求
- 验证所有代码生成功能正常工作

## 非目标（超出范围）
- 修改 dejavu-engine 的核心架构
- 改变 XCell 的代码生成逻辑和输出格式
- 优化代码生成的性能

## 背景与上下文
- XCell 目前在部分代码生成模块中使用了手工字符串拼接，例如 Unity 的 `write_class` 方法
- 部分模块已经开始使用 `#[derive(Template)]` 宏，例如 Cocos 的代码生成
- dejavu-engine 已经提供了 `Template` 宏的基本实现，但可能需要一些改进来支持 XCell 的所有需求

## 功能需求
- **FR-1**：所有代码生成模块必须使用 `#[derive(Template)]` 宏
- **FR-2**：禁止使用手工字符串拼接进行代码生成
- **FR-3**：确保 dejavu-engine 提供足够的特性支持 XCell 的代码生成需求
- **FR-4**：验证所有代码生成功能正常工作

## 非功能需求
- **NFR-1**：代码生成的性能不应显著下降
- **NFR-2**：代码生成的输出格式应与当前保持一致
- **NFR-3**：代码生成的错误处理应清晰明确

## 约束
- **技术**：使用 Rust 语言，依赖 dejavu-engine 和 dejavu-macros
- **业务**：保持与现有代码生成逻辑的兼容性
- **依赖**：依赖 dejavu-engine 的 `Template` 宏实现

## 假设
- dejavu-engine 的 `Template` 宏已经具备基本功能
- XCell 的代码生成逻辑保持不变，只是改变实现方式

## 验收标准

### AC-1：所有代码生成模块使用 Template 宏
- **给定**：XCell 项目的代码生成模块
- **当**：检查代码生成实现
- **然后**：所有模块都使用 `#[derive(Template)]` 宏，没有使用手工字符串拼接
- **验证**：`human-judgment`

### AC-2：dejavu-engine 提供必要特性
- **给定**：dejavu-engine 项目
- **当**：检查 Template 宏的实现
- **然后**：Template 宏支持 XCell 代码生成所需的所有特性
- **验证**：`programmatic`

### AC-3：代码生成功能正常
- **给定**：XCell 项目
- **当**：运行测试脚本 `generate-table.mjs`
- **然后**：所有代码生成功能正常，没有错误
- **验证**：`programmatic`

## 未解决的问题
- [ ] dejavu-engine 的 Template 宏是否支持所有 XCell 代码生成所需的特性？
- [ ] 是否需要对 dejavu-engine 进行修改以支持 XCell 的需求？
- [ ] 如何确保所有代码生成模块都使用 Template 宏，而不是手工字符串拼接？