# 进阶

本部分冊容适用於盡 XCell 有基本了解，希望深入了解其冊部机制、擴充功能或導现高级設定的開發者。

## 文件结构

### 類型系統
- [type-system.md](type-system.md) - 類型系統文件
  - 基本類型（整数、浮点数、布屬、字元串）
  - 奧合類型（陣列、向量、字典、元組）
  - 特殊類型（顏色、时间）
  - 自訂類型（列舉、結構體）
  - 類型转换和驗證

### 欄位元约束
- [key-field.md](key-field.md) - 欄位元约束文件
  - 唯一约束
  - 主鍵约束
  - 奧合约束

### 參照類型
- [ref-type.md](ref-type.md) - 參照類型文件
  - 基本格式
  - 工作原理
  - 使用場景
  - 參照驗證

### 元屬性
- [meta-data.md](meta-data.md) - 元屬性文件
  - 基本元屬性（var, type, default, field, client, server, meta）
  - 資料表格類型標記（class, enum, table, language）
  - 使用规創和範例

### 設定
- [config.md](config.md) - 設定檔案文件
  - 專案設定
  - 資料表格設定
  - 行對應

### 擴充性
- [extensibility.md](extensibility.md) - 擴充性文件
  - 自訂類型系統
  - 擴充程式碼生成器（支援任何编程語言）
  - 為其他遊戲引擎建立匯出器
  - 外掛程式開發指南

## 多引擎支援

XCell 的设计理念是為所有遊戲引擎提供優秀的設定資料表管理解決方案：

- **Unity (C#)**: 完整冊置支援
- **Cocos Creator**: 通过 JSON/XML + TypeScript/Lua 載入器
- **Godot**: 通过 JSON + GDScript 載入器
- **Unreal Engine**: 通过二進位元 + C++ 載入器
- **自訂引擎**: 通过擴充性文件建立自訂匯出器

## 前置要求

在阅读本部分文件之前，建議您：

1. 熟悉 XCell 的基本使用方法
2. 了解專案設定檔案 `XCell.toml` 的基本结构
3. 具獎目标語言编程基础（Rust、C#、C++、Python、Lua、TypeScript 等）
