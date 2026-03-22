# 核心概念

本章节介绍 XCell 配置表管理工具的核心概念，帮助您理解 XCell 的基本原理和工作流程。

## 目录

- [dict 表](./dict.md)：最常用的配置形式，字符串主键
- [list 表](./list.md)：整数 ID 主键，按顺序访问
- [enum 表](./enum.md)：枚举类型，附加额外数据
- [class 表](./class.md)：全局配置类，单例模式
- [language 表](./language.md)：多语言支持
- [合表规则](./merge.md)：表格合并规则

## 表格格式

XCell 采用统一的三行表格格式：

| 行号 | 内容 |
|------|------|
| 第一行 | 字段注释 |
| 第二行 | 字段名（可带类型标记） |
| 第三行 | 字段类型 |
| 第四行及以后 | 数据行 |

> 如果是旧表改造，可通过 line 映射调整行顺序，参见 [配置文件](./config.md#line-映射)。

## 类型标记

类型标记放在第一行第一列，用于声明表格类型：

| 标记 | 说明 |
|------|------|
| 无标记 | 默认为 dict 表 |
| `@dict` | 显式声明 dict 表 |
| `@list` | 声明 list 表 |
| `@enum` | 声明 enum 表 |
| `@class` | 声明 class 表 |
| `@language` | 声明 language 表 |

可通过 `@类型 名称` 指定生成的名称，如 `@enum MonsterType`。

## 快速入门

1. [dict 表](./dict.md)：默认使用 dict 表即可满足大部分需求
2. [list 表](./list.md)：需要整数 ID 时使用 list 表
3. [enum 表](./enum.md)：需要枚举时使用 enum 表
4. [class 表](./class.md)：全局配置使用 class 表
5. [language 表](./language.md)：多语言支持使用 language 表

## 进阶

- [类型系统](../advanced/type-system.md)：基本类型、复合类型
- [引用类型](../advanced/ref-type.md)：表间关联关系
- [元属性](../advanced/meta-data.md)：字段元数据配置
- [扩展性](../advanced/extensibility.md)：自定义类型、代码生成器
- [高级功能](../advanced/advanced-features.md)：高级配置选项
