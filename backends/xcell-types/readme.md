# xcell-types

## 项目简介

xcell-types 是 XCell 项目的核心类型定义库，包含所有数据类型的定义和处理逻辑。

## 模块职责

- 定义基础数据类型，如整数、小数、字符串、布尔值、向量等
- 提供数据类型的序列化和反序列化逻辑
- 定义数据类型的解析和验证规则
- 提供数据类型的代码生成支持

## 主要类型

- **ByteOrder**：字节序枚举，用于指定数据的字节序
- **XCellTyped**：XCell 类型枚举，支持多种数据类型
- **XCellValue**：XCell 值枚举，表示各种类型的数据值
- **TypeMetaInfo**：类型元信息结构体，包含各种类型的描述信息

## 架构位置

xcell-types 是 XCell 项目的最底层模块，被其他所有模块依赖：

1. **xcell-types**：核心类型定义库（当前模块）
2. **xcell-provider**：表格提供者库
3. **xcell-core**：核心分析库
4. **xcell-generator**：代码生成器
5. **xcell**：命令行工具
6. **xcell-macros**：宏定义库
