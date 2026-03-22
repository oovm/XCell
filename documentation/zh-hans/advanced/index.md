# 进阶主题

欢迎使用 XCell 配置表管理工具的进阶主题文档！本部分内容适用于对 XCell 有基本了解，希望深入了解其内部机制、扩展功能或实现高级配置的开发者。

## 文档结构

### 核心概念
- [type-system.md](type-system.md) - 类型系统文档
  - 基本类型（整数、浮点数、布尔、字符串）
  - 复合类型（数组、向量、字典、元组）
  - 特殊类型（颜色、时间）
  - 自定义类型（枚举、结构体）
  - 类型转换和验证

- [key-field.md](key-field.md) - 主键字段文档
  - 默认规则
  - 显式标记
  - 主键类型

- [ref-type.md](ref-type.md) - 引用类型文档
  - 基本格式
  - 工作原理
  - 使用场景
  - 引用验证

- [meta-data.md](meta-data.md) - 元属性文档
  - 基本元属性（var, type, default, field, client, server, meta）
  - 表格类型标记（class, enum, table, language）
  - 使用规则和示例

### 扩展性
- [extensibility.md](extensibility.md) - 扩展性文档
  - 自定义类型系统
  - 扩展代码生成器（支持任何编程语言）
  - 为其他游戏引擎创建导出器
  - 插件开发指南

### 高级功能
- [advanced-features.md](advanced-features.md) - 高级功能文档
  - 高级配置选项
  - 合表规则详解
  - 性能优化技巧
  - 引擎集成概览

### 引擎集成
- [unity.md](unity.md) - Unity 集成文档
  - 配置选项
  - 生成的代码结构
  - 二进制数据加载
  - 类型映射
  - XLua 集成
  - 性能优化

- [cocos.md](cocos.md) - Cocos 集成文档
  - 配置选项
  - 生成的代码结构
  - 数据加载
  - 类型映射
  - 性能优化

## 多引擎支持

XCell 的设计理念是为所有游戏引擎提供优秀的配置表管理解决方案：

- **Unity (C#)**: 完整内置支持，详见 [Unity 集成文档](unity.md)
- **Cocos Creator**: 通过 JSON/XML + TypeScript/Lua 加载器，详见 [Cocos 集成文档](cocos.md)
- **Godot**: 通过 JSON + GDScript 加载器
- **Unreal Engine**: 通过二进制 + C++ 加载器
- **自定义引擎**: 通过扩展性文档创建自定义导出器

## 前置要求

在阅读本部分文档之前，建议您：

1. 熟悉 XCell 的基本使用方法
2. 了解项目配置文件 `XCell.toml` 的基本结构
3. 具备目标语言编程基础（Rust、C#、C++、Python、Lua、TypeScript 等）

## 快速索引

- 想要自定义数据类型？ → [扩展性文档](extensibility.md)
- 需要了解高级配置选项？ → [高级功能文档](advanced-features.md)
- 希望为其他引擎创建导出器？ → [扩展性文档 > 扩展代码生成器](extensibility.md#扩展代码生成器)
- 想了解如何适配 Cocos/Godot/Unreal？ → [高级功能文档](advanced-features.md)
