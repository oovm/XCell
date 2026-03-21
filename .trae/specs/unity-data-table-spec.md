# Unity 数据表生成问题分析 - 产品需求文档

## Overview
- **Summary**: 分析 test-rpg.mjs 脚本生成的 Unity 数据表文件问题，目前只生成了类型定义文件，缺少加载器和数据。
- **Purpose**: 解决 Unity 数据表生成不完整的问题，确保生成完整的加载器和数据文件。
- **Target Users**: 开发人员，特别是使用 XCell 工具生成 Unity 数据表的用户。

## Goals
- 分析当前 Unity 数据表生成的问题原因
- 制定解决方案，确保生成完整的加载器和数据文件
- 验证解决方案的有效性

## Non-Goals (Out of Scope)
- 修改 XCell 工具的源代码
- 改变现有的文件生成结构
- 处理 Cocos 平台的生成问题

## Background & Context
- test-rpg.mjs 脚本执行 xcell 命令生成 Unity 和 Cocos 的数据表文件
- 当前 Unity 生成的文件只有类型定义，如 ItemTable_1774093602.cs，缺少加载器和数据
- Cocos 平台的生成情况未知，需要进一步检查

## Functional Requirements
- **FR-1**: 分析 xcell 工具的生成逻辑，了解为什么只生成类型定义
- **FR-2**: 制定解决方案，确保生成完整的 Unity 数据表加载器
- **FR-3**: 验证生成的 Unity 数据表加载器能够正确加载和使用数据

## Non-Functional Requirements
- **NFR-1**: 解决方案应该保持与现有 XCell 工具的兼容性
- **NFR-2**: 生成的代码应该符合 Unity C# 代码规范
- **NFR-3**: 加载器应该能够高效加载和访问数据表

## Constraints
- **Technical**: 基于现有的 XCell 工具生成逻辑
- **Dependencies**: 依赖 xcell.exe 可执行文件

## Assumptions
- xcell 工具应该能够生成完整的数据表加载器和数据
- Unity 项目结构正确，能够接收生成的文件

## Acceptance Criteria

### AC-1: 分析问题原因
- **Given**: 运行 test-rpg.mjs 脚本
- **When**: 检查 Unity 生成的文件
- **Then**: 确认只生成了类型定义文件，缺少加载器和数据
- **Verification**: `human-judgment`

### AC-2: 制定解决方案
- **Given**: 分析 xcell 工具的生成逻辑
- **When**: 制定解决方案
- **Then**: 解决方案应该能够生成完整的 Unity 数据表加载器和数据
- **Verification**: `human-judgment`

### AC-3: 验证解决方案
- **Given**: 实施解决方案
- **When**: 运行 test-rpg.mjs 脚本
- **Then**: Unity 生成的文件应该包含完整的加载器和数据
- **Verification**: `programmatic`

## Open Questions
- [ ] xcell 工具的生成配置在哪里？
- [ ] Unity 项目的数据表结构应该是怎样的？
- [ ] 如何确保生成的数据表加载器能够正确工作？