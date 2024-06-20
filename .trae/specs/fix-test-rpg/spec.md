# Fix test-rpg.mjs - Product Requirement Document

## Overview
- **Summary**: 修复 `test-rpg.mjs` 脚本中的问题，确保其在 Windows 环境下正确运行，能够找到并执行 xcell 可执行文件，以及正确处理命令执行结果。
- **Purpose**: 确保测试脚本能够可靠地测试 XCell 项目的 RPG 示例，提高开发和测试效率。
- **Target Users**: XCell 项目的开发人员和测试人员。

## Goals
- 修复脚本中的路径处理问题，确保在 Windows 环境下正确解析路径
- 增强错误处理，提供更清晰的错误信息
- 确保脚本能够正确找到 xcell 可执行文件
- 验证 xcell generate 命令的执行结果

## Non-Goals (Out of Scope)
- 不修改 xcell 可执行文件本身
- 不修改 RPG 示例项目的内容
- 不添加新的测试功能

## Background & Context
- 当前的 `test-rpg.mjs` 脚本在 Windows 环境下可能存在路径处理问题
- 脚本使用 `execSync` 执行命令，但错误处理较为简单
- 脚本没有验证生成结果的正确性

## Functional Requirements
- **FR-1**: 脚本应能在 Windows 环境下正确解析路径
- **FR-2**: 脚本应能正确找到 xcell 可执行文件
- **FR-3**: 脚本应能执行 xcell generate 命令并处理执行结果
- **FR-4**: 脚本应提供清晰的错误信息

## Non-Functional Requirements
- **NFR-1**: 脚本应具有良好的跨平台兼容性
- **NFR-2**: 脚本应具有良好的错误处理能力
- **NFR-3**: 脚本应提供清晰的日志输出

## Constraints
- **Technical**: 必须在 Windows 环境下运行，使用 Node.js
- **Dependencies**: 依赖 xcell 可执行文件

## Assumptions
- xcell 可执行文件已通过 `cargo build` 编译
- RPG 示例项目存在于 `examples/rpg` 目录

## Acceptance Criteria

### AC-1: 路径处理正确
- **Given**: 在 Windows 环境下运行脚本
- **When**: 脚本解析路径
- **Then**: 脚本应能正确解析路径，找到 xcell 可执行文件
- **Verification**: `programmatic`

### AC-2: 错误处理完善
- **Given**: 当 xcell 可执行文件不存在时
- **When**: 运行脚本
- **Then**: 脚本应提供清晰的错误信息并退出
- **Verification**: `programmatic`

### AC-3: 命令执行成功
- **Given**: xcell 可执行文件存在且 RPG 示例项目正确
- **When**: 运行脚本
- **Then**: 脚本应成功执行 xcell generate 命令
- **Verification**: `programmatic`

### AC-4: 执行结果验证
- **Given**: xcell generate 命令执行成功
- **When**: 脚本执行完成
- **Then**: 脚本应验证生成结果并提供成功信息
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要支持 Release 版本的 xcell 可执行文件？
- [ ] 是否需要添加更多的日志信息？
- [ ] 是否需要支持其他操作系统？