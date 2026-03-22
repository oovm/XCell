# 文档多语言翻译与检查规范

## Why

XCell 项目的中文文档 (`zh-hans`) 需要翻译为多种语言，以便更多用户使用。同时需要一个检查脚本来确保各语言版本的文档结构一致，避免翻译遗漏或结构偏差。

## What Changes

- 将 `documentation/zh-hans` 目录下的所有 Markdown 文档翻译为多种语言
- 创建 `scripts/check-document.mjs` 脚本检查文档完整性
- 支持检查：文件数量一致、文件名对应、Header 结构一致

## Impact

- Affected code: `documentation/` 目录下的所有语言版本
- New file: `scripts/check-document.mjs`

## ADDED Requirements

### Requirement: 多语言文档翻译

系统 SHALL 提供以下语言版本的文档：
- `zh-hans` - 简体中文（源语言）
- `en` - 英语
- `ja` - 日语
- `zh-hant` - 繁体中文

#### Scenario: 文档目录结构
- **GIVEN** 文档目录 `documentation/`
- **WHEN** 查看目录结构
- **THEN** 每种语言都有相同的子目录结构：
  - `overview/` - 概述
  - `use-cases/` - 使用案例
  - `advanced/` - 进阶内容
  - `maintainer/` - 维护者文档
    - `architecture/` - 架构文档
    - `targets/` - 目标平台文档

### Requirement: 文档完整性检查脚本

系统 SHALL 提供 `scripts/check-document.mjs` 脚本用于检查文档完整性。

#### Scenario: 检查文件数量
- **GIVEN** 多语言文档目录
- **WHEN** 运行检查脚本
- **THEN** 验证所有语言版本的文件数量与源语言一致

#### Scenario: 检查文件名对应
- **GIVEN** 多语言文档目录
- **WHEN** 运行检查脚本
- **THEN** 验证所有语言版本的文件名与源语言一一对应

#### Scenario: 检查 Header 结构
- **GIVEN** 多语言文档目录
- **WHEN** 运行检查脚本
- **THEN** 验证所有语言版本的 Markdown Header 结构与源语言一致
  - Header 层级数量相同
  - Header 顺序相同
  - Header 文本已翻译（非空）

#### Scenario: 输出检查报告
- **GIVEN** 检查完成
- **WHEN** 存在问题
- **THEN** 输出详细的错误报告，包括：
  - 缺失的文件列表
  - 多余的文件列表
  - Header 结构不一致的文件及具体差异
- **WHEN** 检查通过
- **THEN** 输出成功消息

### Requirement: 脚本使用方式

#### Scenario: 命令行调用
- **GIVEN** 脚本已创建
- **WHEN** 执行 `node scripts/check-document.mjs`
- **THEN** 脚本检查所有语言版本并输出结果

#### Scenario: 指定源语言
- **GIVEN** 脚本已创建
- **WHEN** 执行 `node scripts/check-document.mjs --source zh-hans`
- **THEN** 使用指定的源语言作为基准

#### Scenario: 指定目标语言
- **GIVEN** 脚本已创建
- **WHEN** 执行 `node scripts/check-document.mjs --targets en,ja`
- **THEN** 只检查指定的目标语言

## 文档文件清单

### overview/ 目录
| 文件 | 描述 |
|------|------|
| `index.md` | 功能特性概述 |
| `class.md` | 类表类型说明 |
| `dict.md` | 字典表类型说明 |
| `enum.md` | 枚举表类型说明 |
| `language.md` | 语言表类型说明 |
| `list.md` | 列表表类型说明 |
| `merge.md` | 合表功能说明 |

### use-cases/ 目录
| 文件 | 描述 |
|------|------|
| `index.md` | 使用案例索引 |
| `unity-integration.md` | Unity 集成指南 |

### advanced/ 目录
| 文件 | 描述 |
|------|------|
| `index.md` | 进阶内容索引 |
| `advanced-features.md` | 高级功能 |
| `config.md` | 配置文件说明 |
| `extensibility.md` | 扩展性说明 |
| `key-field.md` | 字段约束 |
| `meta-data.md` | 元属性说明 |
| `ref-type.md` | 引用类型 |
| `type-system.md` | 类型系统 |

### maintainer/architecture/ 目录
| 文件 | 描述 |
|------|------|
| `index.md` | 架构设计文档 |

### maintainer/targets/ 目录
| 文件 | 描述 |
|------|------|
| `index.md` | 目标平台索引 |
| `cocos.md` | Cocos 平台 |
| `godot.md` | Godot 平台 |
| `json.md` | JSON 导出 |
| `react.md` | React 平台 |
| `sql.md` | SQL 导出 |
| `typescript.md` | TypeScript 导出 |
| `unity.md` | Unity 平台 |
| `unreal.md` | Unreal 平台 |
| `vue.md` | Vue 平台 |
| `xlua.md` | XLua 集成 |

### 根目录
| 文件 | 描述 |
|------|------|
| `readme.md` | 快速开始指南 |

## Header 结构检查规则

1. **层级数量**：目标文档的 Header 层级数量必须与源文档相同
2. **层级顺序**：Header 的层级顺序必须一致（如 H1 -> H2 -> H3 的顺序）
3. **文本非空**：每个 Header 的文本必须非空（已翻译）
4. **忽略代码块**：代码块内的 `#` 符号不计入 Header 检查

## 错误输出格式

```
文档检查报告
============

✗ 文件数量不一致
  - zh-hans: 25 个文件
  - en: 23 个文件
  - 缺失: overview/class.md, advanced/type-system.md

✗ Header 结构不一致: en/overview/index.md
  - 源文档 H1 数量: 1
  - 目标文档 H1 数量: 2
  - 差异位置: 第 15 行

✓ ja: 检查通过 (25 个文件)

总计: 2 个语言存在问题
```
