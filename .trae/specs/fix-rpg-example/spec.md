# 修复 RPG 示例项目 - 产品需求文档

## Overview
- **Summary**: 修复 RPG 示例项目中的 CSV 文件格式问题，确保 xcell 工具能够正确处理这些文件并生成 Unity 和 Cocos 的代码和数据文件。
- **Purpose**: 解决 RPG 示例无法正常工作的问题，使开发者能够正常使用 xcell 工具进行数据表格的生成。
- **Target Users**: XCell 项目的开发者和使用者。

## Goals
- 修复 RPG 示例中所有 CSV 文件的格式问题
- 确保 xcell 工具能够正确处理 RPG 示例项目
- 验证 Unity 和 Cocos 代码生成功能正常工作
- 更新 test-rpg.mjs 脚本以正确测试 RPG 示例

## Non-Goals (Out of Scope)
- 不修改 xcell 工具的核心功能
- 不添加新的示例项目
- 不修改 RPG 示例的游戏逻辑数据

## Background & Context
- 当前的 RPG 示例项目存在以下问题：
  1. ProjectSettings.toml 配置中 include 字段设置为 "*.xlsx"，但实际文件是 CSV 格式
  2. MonsterType.csv 文件缺少类型定义行（第二行）
  3. 可能还有其他 CSV 文件格式问题
- 当运行 xcell 时，会出现 "MapAccess::next_value: missing field `name` in `enumerate`" 错误
- cocos 和 unity 目录是空的，因为 xcell 没有成功生成文件

## Functional Requirements
- **FR-1**: 修复 ProjectSettings.toml 配置，支持 CSV 文件
- **FR-2**: 修复所有 RPG 示例 CSV 文件的格式，确保符合 xcell 要求
- **FR-3**: 确保 xcell 能够成功处理 RPG 示例项目
- **FR-4**: 验证 Unity 代码生成功能正常
- **FR-5**: 验证 Cocos 代码生成功能正常（可选，配置中当前禁用）

## Non-Functional Requirements
- **NFR-1**: 修复后的示例应能在 Windows 环境下正常运行
- **NFR-2**: 生成的代码文件应符合预期格式
- **NFR-3**: 错误信息应清晰明确

## Constraints
- **Technical**: 必须保持与现有 xcell 工具兼容
- **Dependencies**: 依赖 xcell 工具正确编译

## Assumptions
- xcell 工具本身没有 bug，问题仅在示例文件格式
- rpg-typed 示例的文件格式是正确的参考
- 现有的 CSV 文件数据内容是正确的，只是格式需要调整

## Acceptance Criteria

### AC-1: ProjectSettings.toml 配置正确
- **Given**: RPG 示例项目存在
- **When**: 查看 ProjectSettings.toml
- **Then**: include 配置应支持 CSV 文件，Unity 生成启用
- **Verification**: `programmatic`

### AC-2: 所有 CSV 文件格式正确
- **Given**: RPG 示例项目的所有 CSV 文件
- **When**: 检查文件格式
- **Then**: 每个文件都有正确的表头、类型定义行和数据行
- **Verification**: `programmatic`

### AC-3: xcell 能成功运行
- **Given**: 修复后的 RPG 示例项目
- **When**: 运行 xcell 工具
- **Then**: xcell 应成功执行，无错误
- **Verification**: `programmatic`

### AC-4: Unity 代码生成成功
- **Given**: xcell 成功运行
- **When**: 检查生成的文件
- **Then**: Unity 相关的代码和数据文件应生成在正确位置
- **Verification**: `programmatic`

### AC-5: 测试脚本运行成功
- **Given**: 修复后的 RPG 示例项目
- **When**: 运行 test-rpg.mjs
- **Then**: 测试脚本应成功执行
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要启用 Cocos 生成功能？
