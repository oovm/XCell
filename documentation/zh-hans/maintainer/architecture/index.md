# XCell 架构设计文档

## 1. 项目整体架构说明

XCell 是一个配置表管理工具，采用 Rust 编写，采用模块化设计，各模块职责清晰，耦合度低。项目整体分为以下几个主要部分：

- 核心库 (`xcell-core`)：负责核心业务逻辑
- 命令行工具 (`xcell-cli`)：提供命令行交互接口
- 图形界面 (`xcell-gui`)：提供图形用户界面（基于 Tauri）
- 类型系统 (`xcell-types`)：定义数据类型和转换
- 错误处理 (`xcell-errors`)：统一错误处理机制
- 流式 IO (`stream-io`)：提供二进制数据读写功能

### 技术栈

- **后端**：Rust
- **GUI**：Tauri + Vue.js
- **Excel 读取**：calamine
- **模板引擎**：askama
- **异步运行时**：tokio

## 2. 模块划分与职责说明

### 2.1 xcell-core - 核心业务模块

**职责**：
- 管理工作空间和配置
- Excel 表格读取与解析
- 配置表数据处理
- 代码生成（C#、二进制、XML）
- 合表规则处理
- 文件监控

**核心文件**：
- `projects/xcell-core/src/lib.rs` - 模块导出和公共 API
- `projects/xcell-core/src/config/mod.rs` - 配置管理与工作空间管理器
- `projects/xcell-core/src/x_table/mod.rs` - 表格数据结构定义
- `projects/xcell-core/src/codegen/mod.rs` - 代码生成器

**核心组件**：
- `WorkspaceManager` - 工作空间管理器，负责协调整个工作流程
- `CalamineTable` - Excel 表格读取器
- `XClassTable` - 类表类型
- `XDictTable` - 字典表类型
- `XEnumerateTable` - 枚举表类型
- `XLanguageTable` - 语言表类型

### 2.2 xcell-cli - 命令行工具

**职责**：
- 提供命令行接口
- 解析命令行参数
- 调用 xcell-core 执行任务

**核心文件**：
- `projects/xcell-cli/src/main.rs` - 程序入口点
- `projects/xcell-cli/src/lib.rs` - 命令行参数定义

**主要功能**：
- 检查配置 (`check`)
- 清理输出 (`clear`)
- 启用/禁用 XML 输出
- 启用/禁用 JSON 输出
- 文件监控模式 (`watch`)

### 2.3 xcell-gui - 图形用户界面

**职责**：
- 提供友好的图形用户界面
- 与 Tauri 后端集成
- 调用 xcell-core 功能

**核心文件**：
- `projects/xcell-gui/src-tauri/src/main.rs` - Tauri 应用入口
- `projects/xcell-gui/src/App.vue` - Vue 应用组件

**技术框架**：
- Tauri 作为桌面应用框架
- Vue.js 作为前端框架
- Tailwind CSS 用于样式

### 2.4 xcell-types - 类型系统

**职责**：
- 定义所有数据类型
- 提供类型转换和解析
- 支持 C# 代码生成的类型映射

**核心文件**：
- `projects/xcell-types/src/lib.rs` - 模块导出
- `projects/xcell-types/src/typing/mod.rs` - 类型定义
- `projects/xcell-types/src/value/mod.rs` - 值处理

**支持的类型**：
- 整数类型 (Integer)
- 小数类型 (Decimal)
- 布尔类型 (Boolean)
- 字符串类型 (String)
- 数组类型 (Array)
- 向量类型 (Vector)
- 语言类型 (Language)
- 枚举类型 (Enumerate)
- 颜色类型 (Color)
- 时间类型 (Time)

### 2.5 xcell-errors - 错误处理

**职责**：
- 统一错误类型定义
- 错误转换和包装
- 第三方库错误适配

**核心文件**：
- `projects/xcell-errors/src/lib.rs` - 模块导出
- `projects/xcell-errors/src/errors/mod.rs` - 错误定义

**主要功能**：
- `XError` - 统一错误类型
- `XResult<T>` - 统一结果类型
- `Validation<T>` - 验证结果类型
- 第三方库错误适配器 (for_3rd)

### 2.6 stream-io - 流式 IO

**职责**：
- 提供二进制数据流读写功能
- 支持大端序和小端序
- 高效的二进制操作

**核心文件**：
- `projects/stream-io/src/lib.rs` - 模块导出
- `projects/stream-io/src/standard/mod.rs` - 标准流实现

**主要功能**：
- `StreamReader` - 二进制流读取器
- `StreamWriter` - 二进制流写入器
- `ByteOrder` - 字节序枚举（BigEndian/LittleEndian）

## 3. 数据流说明

### 3.1 整体流程

从 Excel 表格读取到代码导出的完整流程如下：

```
Excel 文件 → 读取解析 → 表格识别 → 数据处理 → 代码生成 → 输出文件
```

### 3.2 详细步骤

#### 步骤 1: 初始化工作空间

1. 解析命令行参数 (`projects/xcell-cli/src/main.rs:8`)
2. 创建 `WorkspaceManager` 实例 (`projects/xcell-core/src/config/mod.rs:64`)
3. 加载项目配置 (`ProjectConfig`)

#### 步骤 2: 扫描文件

1. 首次扫描工作目录 (`projects/xcell-core/src/config/mod.rs:78`)
2. 使用 `WalkDir` 遍历目录
3. 根据配置的 `include` 模式过滤文件

#### 步骤 3: 读取 Excel 文件

1. 使用 `CalamineTable::load()` 读取 Excel 文件 (`projects/xcell-core/src/config/mod.rs:122`)
2. 解析表格头部 (`XCellHeader`)
3. 读取所有数据行

#### 步骤 4: 识别表格类型

依次尝试识别以下表格类型 (`projects/xcell-core/src/config/mod.rs:123-155`)：

1. `XListTable` - 列表表
2. `XDictTable` - 字典表
3. `XEnumerateTable` - 枚举表
4. `XClassTable` - 类表
5. `XLanguageTable` - 语言表
6. `XLanguageID` - 语言 ID 表

#### 步骤 5: 处理表格数据

根据表格类型执行相应操作：

- 对于枚举表：添加到 `DefineManager`
- 对于语言表：添加到 `LanguageManager`
- 对于其他表：进行数据验证和存储

#### 步骤 6: 链接枚举

调用 `link_enumerate()` 方法将枚举定义链接到对应的数据字段

#### 步骤 7: 代码生成

调用 `write_unity()` 方法 (`projects/xcell-core/src/config/mod.rs:158`)：

1. 生成二进制文件 (`projects/xcell-core/src/codegen/unity/binary.rs`)
2. 生成 C# 代码 (`projects/xcell-core/src/codegen/unity/class.rs` 等)
3. 生成 XML 配置 (`projects/xcell-core/src/codegen/xml/mod.rs`)

#### 步骤 8: 文件监控（可选）

如果启用了 `--watch` 参数：
1. 启动文件监控器 (`projects/xcell-core/src/config/mod.rs:99`)
2. 监听文件变更
3. 自动重新处理变更的文件

## 4. 核心代码位置引用

### 工作空间管理
- `WorkspaceManager::new()` - `projects/xcell-core/src/config/mod.rs:64`
- `WorkspaceManager::first_walk()` - `projects/xcell-core/src/config/mod.rs:78`
- `WorkspaceManager::load_file()` - `projects/xcell-core/src/config/mod.rs:116`
- `WorkspaceManager::write_unity()` - `projects/xcell-core/src/config/mod.rs:158`

### 表格读取
- `CalamineTable::load()` - `projects/xcell-core/src/x_table/table/mod.rs`
- `XCellHeader` - `projects/xcell-core/src/x_table/header/mod.rs`

### 表格类型
- `XClassTable` - `projects/xcell-core/src/x_table/class/mod.rs`
- `XDictTable` - `projects/xcell-core/src/x_table/dictionary/mod.rs`
- `XEnumerateTable` - `projects/xcell-core/src/x_table/enumerate/mod.rs`
- `XLanguageTable` - `projects/xcell-core/src/x_table/language/mod.rs`

### 代码生成
- `UnityCodegen` - `projects/xcell-core/src/config/unity/mod.rs`
- C# 类生成 - `projects/xcell-core/src/codegen/unity/class.rs`
- 二进制生成 - `projects/xcell-core/src/codegen/unity/binary.rs`
- XML 生成 - `projects/xcell-core/src/codegen/xml/mod.rs`

### 类型系统
- `TypeDescription` - `projects/xcell-types/src/typing/mod.rs`
- `XCellValue` - `projects/xcell-types/src/value/mod.rs`
- `CSharpReader`/`CSharpWriter` - `projects/xcell-types/src/codegen/csharp_ffi/mod.rs`

### 错误处理
- `XError` - `projects/xcell-errors/src/errors/mod.rs`
- `XResult` - `projects/xcell-errors/src/lib.rs:8`

### 流式 IO
- `StreamReader` - `projects/stream-io/src/standard/reader/mod.rs`
- `StreamWriter` - `projects/stream-io/src/standard/writer/mod.rs`

## 5. 扩展开发指南

### 添加新的表格类型

1. 在 `projects/xcell-core/src/x_table/` 下创建新模块
2. 实现 `confirm()` 方法用于识别表格
3. 实现 `perform()` 方法用于处理表格数据
4. 在 `WorkspaceManager::try_perform_file()` 中添加识别逻辑

### 添加新的数据类型

1. 在 `projects/xcell-types/src/` 下创建新模块
2. 实现类型解析和转换逻辑
3. 在 `projects/xcell-types/src/lib.rs` 中导出
4. 添加 C# 代码生成支持

### 添加新的代码生成器

1. 在 `projects/xcell-core/src/codegen/` 下创建新模块
2. 实现代码生成逻辑
3. 在 `WorkspaceManager::write_unity()` 中调用
