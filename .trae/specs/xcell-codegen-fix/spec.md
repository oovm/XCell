# XCell 代码生成问题修复 - 产品需求文档

## 概述
- **Summary**: 修复 XCell 在运行 RPG 示例时生成占位符文件而不是实际代码的问题，聚合所有代码生成能力到 xcell-generator 模块，使用模板系统生成代码。
- **Purpose**: 解决代码生成不完整的问题，确保 XCell 能够正确生成 Cocos 和 Unity 平台的代码文件。
- **Target Users**: XCell 开发者和使用 XCell 的游戏开发团队。

##  Goals
- 修复占位符文件生成问题，确保生成实际的代码文件
- 聚合所有代码生成能力到 xcell-generator 模块
- 使用 templates 目录管理代码生成模板
- 使用 derive Template 调用模板系统
- 确保 Cocos 和 Unity 平台的代码生成正常工作

##  Non-Goals (Out of Scope)
- 不修改现有的表结构和数据
- 不修改项目配置文件格式
- 不添加新的代码生成平台

##  Background & Context
- XCell 是一个表格数据驱动的代码生成工具，用于游戏开发中处理数据表格
- 目前在运行 RPG 示例时，生成的是占位符文件（Placeholder.ts 和 Placeholder.cs），而不是实际的代码文件
- 代码生成能力分散在多个模块中，需要聚合到 xcell-generator 模块
- 项目已经有 templates 目录用于管理代码生成模板

##  Functional Requirements
- **FR-1**: 修复占位符文件生成问题，确保生成实际的代码文件
- **FR-2**: 聚合所有代码生成能力到 xcell-generator 模块
- **FR-3**: 使用 templates 目录管理代码生成模板
- **FR-4**: 使用 derive Template 调用模板系统
- **FR-5**: 确保 Cocos 平台的代码生成正常工作
- **FR-6**: 确保 Unity 平台的代码生成正常工作

##  Non-Functional Requirements
- **NFR-1**: 代码生成性能应保持在合理范围内
- **NFR-2**: 代码生成结果应符合各平台的代码规范
- **NFR-3**: 代码生成过程应具有良好的错误处理能力

##  Constraints
- **Technical**: 必须使用 Rust 语言实现，必须使用 dejavu 模板系统
- **Dependencies**: 依赖 xcell-analyzer 提供的工作区管理和表格分析功能

##  Assumptions
- 项目的 templates 目录已经包含了所需的代码生成模板
- 表格文件的格式和结构是正确的
- 项目配置文件是有效的

##  Acceptance Criteria

###  AC-1: 修复占位符文件生成问题
- **Given**: 运行 xcell 命令处理 RPG 示例目录
- **When**: 检查生成的文件
- **Then**: 生成的文件应该是实际的代码文件，而不是占位符文件
- **Verification**: `programmatic`

###  AC-2: 聚合代码生成能力
- **Given**: 检查 xcell-generator 模块
- **When**: 查看代码结构
- **Then**: 所有代码生成能力应该集中在 xcell-generator 模块中
- **Verification**: `human-judgment`

###  AC-3: 使用模板系统
- **Given**: 检查代码生成过程
- **When**: 查看模板使用情况
- **Then**: 代码生成应该使用 templates 目录中的模板文件，并通过 derive Template 调用
- **Verification**: `human-judgment`

###  AC-4: Cocos 平台代码生成
- **Given**: 运行 xcell 命令处理 RPG 示例目录
- **When**: 检查 Cocos 目录中的生成文件
- **Then**: 应该生成正确的 TypeScript 代码文件
- **Verification**: `programmatic`

###  AC-5: Unity 平台代码生成
- **Given**: 运行 xcell 命令处理 RPG 示例目录
- **When**: 检查 Unity 目录中的生成文件
- **Then**: 应该生成正确的 C# 代码文件
- **Verification**: `programmatic`

##  Open Questions
- [ ] 为什么当前会生成占位符文件而不是实际代码？
- [ ] 如何确保模板系统正确工作？
- [ ] 如何处理不同平台的代码生成差异？