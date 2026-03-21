# XCell 代码生成能力聚合 - 产品需求文档

## 概述
- **摘要**：将 XCell 项目中分散的代码生成能力聚合到 `xcell-generator` 模块，使用统一的模板系统管理，确保所有平台的代码生成都通过 `xcell-generator` 执行。
- **目的**：解决当前代码生成逻辑分散、重复实现、生成占位符文件等问题，提供统一、可维护的代码生成架构。
- **目标用户**：XCell 项目维护者和使用 XCell 生成代码的开发者。

## 目标
- 聚合所有代码生成能力到 `xcell-generator` 模块
- 使用 `xcell-generator/templates` 目录管理所有模板
- 实现基于模板的代码生成机制
- 修复 `test-rpg.mjs` 运行时生成占位符文件的问题
- 确保 Cocos 和 Unity 平台的代码生成正常工作

## 非目标（范围外）
- 不修改现有的数据结构和验证逻辑
- 不改变现有的命令行接口
- 不添加新的代码生成格式

## 背景与上下文
- 当前 XCell 项目的代码生成逻辑分散在多个模块中
- `xcell` 可执行文件在 `main.rs` 中硬编码了简单的代码生成逻辑
- `WorkspaceManager` 的 `write_unity()` 和 `write_cocos()` 方法是空的
- `xcell-generator` 模块已经实现了完整的代码生成能力，但没有被调用
- 运行 `test-rpg.mjs` 时生成的是占位符文件，而不是实际的数据表代码

## 功能需求
- **FR-1**：修改 `xcell` 可执行文件，使其使用 `xcell-generator` 模块进行代码生成
- **FR-2**：实现基于模板的代码生成机制，使用 `xcell-generator/templates` 目录管理模板
- **FR-3**：确保 Cocos 平台的代码生成正常工作，生成正确的 TypeScript 文件
- **FR-4**：确保 Unity 平台的代码生成正常工作，生成正确的 C# 文件
- **FR-5**：修复 `test-rpg.mjs` 运行时生成占位符文件的问题

## 非功能需求
- **NFR-1**：代码生成性能应保持稳定，不应明显增加生成时间
- **NFR-2**：代码生成结果应与之前的生成结果兼容
- **NFR-3**：代码生成过程应具有良好的错误处理和日志记录
- **NFR-4**：模板系统应易于扩展和维护

## 约束
- **技术**：使用 Rust 语言实现，依赖现有的 `xcell-generator` 模块
- **依赖**：保持与现有依赖的兼容性
- **时间**：尽快完成，确保 `test-rpg.mjs` 能够正常运行

## 假设
- 现有的 `xcell-generator` 模块已经实现了大部分代码生成逻辑
- 模板系统已经存在，需要正确使用
- 项目的目录结构和配置文件格式保持不变

## 验收标准

### AC-1：xcell 可执行文件使用 xcell-generator 模块
- **给定**：运行 `xcell` 命令
- **当**：执行代码生成时
- **则**：`xcell` 应调用 `xcell-generator` 模块进行代码生成
- **验证**：`programmatic`

### AC-2：基于模板的代码生成
- **给定**：`xcell-generator/templates` 目录包含模板文件
- **当**：执行代码生成时
- **则**：代码生成应使用模板文件生成代码
- **验证**：`programmatic`

### AC-3：Cocos 平台代码生成
- **给定**：运行 `test-rpg.mjs` 脚本
- **当**：处理 `rpg-typed` 和 `rpg-untyped` 目录时
- **则**：应在 `cocos/assets/scripts/dataTable/generated/` 目录生成正确的 TypeScript 文件
- **验证**：`programmatic`

### AC-4：Unity 平台代码生成
- **给定**：运行 `test-rpg.mjs` 脚本
- **当**：处理 `rpg-typed` 和 `rpg-untyped` 目录时
- **则**：应在 `unity/Assets/Scripts/DataTable/Generated/` 目录生成正确的 C# 文件
- **验证**：`programmatic`

### AC-5：修复占位符文件问题
- **给定**：运行 `test-rpg.mjs` 脚本
- **当**：处理 `rpg-typed` 和 `rpg-untyped` 目录时
- **则**：不应生成 `Placeholder.ts` 和 `Placeholder.cs` 文件，而是生成实际的数据表代码
- **验证**：`programmatic`

## 未解决的问题
- [ ] 确认 `xcell-generator` 模块是否已经完全实现了所有必要的代码生成逻辑
- [ ] 确认模板文件是否已经存在且正确配置
- [ ] 确认 `ProjectSettings.toml` 配置文件是否正确设置了代码生成选项