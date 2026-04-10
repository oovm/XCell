# 工作空间组功能完善 Spec

## Why
xcell-analyzer 和 xcell 命令行工具作为工作空间组的核心模块，当前存在工作空间管理功能不完善、命令行工具描述错误且缺少实用子命令、文件监控功能不完整、表格分析性能有待优化等问题。需要系统性地推进 roadmap 中工作空间组的本月工作重点。

## What Changes
- **完善工作空间管理**：为 `WorkspaceManager` 增加工作空间状态查询、摘要输出、工作空间初始化等方法
- **优化表格分析性能**：移除调试用 `println!`，改用 `tracing`；优化文件格式检测避免全量读取；优化表格类型判断逻辑
- **改进命令行工具**：修复错误的 CLI 描述文档；增加 `info`、`init`、`list` 子命令；增加 `--verbose`/`--quiet` 日志级别控制选项
- **增加更多命令行选项**：增加 `--dry-run` 试运行选项、`--filter` 表格过滤选项、`--parallel` 并行处理选项
- **完善文件监控功能**：支持文件创建/删除事件；增加防抖机制；文件变更后自动触发代码生成；支持优雅退出
- **添加文档注释**：为所有 public 的结构体、枚举、方法、字段添加文档注释

## Impact
- Affected specs: xcell-analyzer, xcell
- Affected code:
  - `backends/xcell-analyzer/src/config/mod.rs`
  - `backends/xcell-analyzer/src/utils/file_format.rs`
  - `backends/xcell-analyzer/src/utils/watcher/mod.rs`
  - `backends/xcell/src/lib.rs`
  - `backends/xcell/src/main.rs`
  - `backends/xcell/src/workspace.rs`

## ADDED Requirements

### Requirement: 工作空间状态查询
系统 SHALL 提供 `WorkspaceManager::status()` 方法，返回工作空间的当前状态信息，包括已加载的表数量、枚举数量、类数量等。

#### Scenario: 查询工作空间状态
- **WHEN** 调用 `WorkspaceManager::status()`
- **THEN** 返回包含表数量、枚举数量、字典数量、类数量、语言数量等信息的结构体

### Requirement: 工作空间摘要输出
系统 SHALL 提供 `info` 子命令，输出工作空间的配置和状态摘要信息。

#### Scenario: 执行 info 子命令
- **WHEN** 用户运行 `xcell info`
- **THEN** 输出工作空间根目录、包含的文件模式、已加载的表数量和类型分布、各生成器启用状态等信息

### Requirement: 工作空间初始化
系统 SHALL 提供 `init` 子命令，在指定目录创建默认的 `ProjectConfig.toml` 配置文件。

#### Scenario: 初始化新工作空间
- **WHEN** 用户在空目录运行 `xcell init`
- **THEN** 在该目录创建包含默认配置的 `ProjectConfig.toml` 文件

#### Scenario: 工作空间已存在
- **WHEN** 用户在已有 `ProjectConfig.toml` 的目录运行 `xcell init`
- **THEN** 提示用户工作空间已存在，不覆盖现有配置

### Requirement: 表格列表查询
系统 SHALL 提供 `list` 子命令，列出工作空间中已加载的所有表格及其类型。

#### Scenario: 列出所有表格
- **WHEN** 用户运行 `xcell list`
- **THEN** 输出表格名称、类型（list/dict/class/enum/language）、文件路径等信息

### Requirement: 日志级别控制
系统 SHALL 支持 `--verbose` 和 `--quiet` 命令行选项控制日志输出级别。

#### Scenario: 启用详细日志
- **WHEN** 用户运行 `xcell --verbose`
- **THEN** 日志级别设置为 TRACE，输出所有调试信息

#### Scenario: 静默模式
- **WHEN** 用户运行 `xcell --quiet`
- **THEN** 日志级别设置为 ERROR，仅输出错误信息

### Requirement: 试运行选项
系统 SHALL 支持 `--dry-run` 命令行选项，仅执行分析和验证，不生成任何输出文件。

#### Scenario: 试运行模式
- **WHEN** 用户运行 `xcell --dry-run`
- **THEN** 执行工作空间加载和表格分析，但不调用代码生成器生成输出文件

### Requirement: 表格过滤选项
系统 SHALL 支持 `--filter` 命令行选项，通过 glob 模式过滤需要处理的表格文件。

#### Scenario: 过滤表格
- **WHEN** 用户运行 `xcell --filter "Item*"`
- **THEN** 仅处理文件名匹配 `Item*` 模式的表格文件

### Requirement: 文件监控支持创建和删除事件
系统 SHALL 在文件监控中处理文件创建和删除事件，而不仅仅是修改事件。

#### Scenario: 新文件创建
- **WHEN** 监控模式下工作空间目录中新增了匹配的表格文件
- **THEN** 自动加载该文件并触发代码生成

#### Scenario: 文件删除
- **WHEN** 监控模式下工作空间目录中的表格文件被删除
- **THEN** 从工作空间中移除该文件的数据并触发代码生成

### Requirement: 文件监控防抖
系统 SHALL 对文件监控事件实现防抖机制，避免短时间内多次触发处理。

#### Scenario: 快速连续修改
- **WHEN** 同一文件在 500ms 内被多次修改
- **THEN** 仅在最后一次修改后的 500ms 延迟后触发一次处理

### Requirement: 文件变更后自动代码生成
系统 SHALL 在文件监控模式下，文件变更并重新加载后自动触发代码生成。

#### Scenario: 文件修改后自动生成
- **WHEN** 监控模式下表格文件被修改并重新加载
- **THEN** 自动调用代码生成器更新输出文件

### Requirement: 修复 CLI 描述文档
系统 SHALL 修复 `XCellArgs` 结构体的错误描述文档，从 "Simple program to greet a person" 改为正确的 XCell 工具描述。

#### Scenario: 检查 CLI 帮助信息
- **WHEN** 用户运行 `xcell --help`
- **THEN** 显示正确的工具描述，如 "XCell 配置表管理工具"

### Requirement: 移除调试输出
系统 SHALL 将 `WorkspaceManager` 中的 `println!` 调试输出替换为 `tracing` 日志调用。

#### Scenario: 正常运行输出
- **WHEN** 执行工作空间加载
- **THEN** 不再出现 `println!` 的调试输出，所有信息通过 `tracing` 框架输出

### Requirement: 优化文件格式检测
系统 SHALL 优化 `FileFormatDetector::detect_by_content` 方法，避免将整个文件内容读入内存，仅读取必要的头部字节。

#### Scenario: 检测大文件格式
- **WHEN** 检测一个大型 CSV 文件的格式
- **THEN** 仅读取文件头部少量字节进行判断，不将整个文件读入内存

### Requirement: 文档注释完整性
所有 public 的结构体、枚举、方法、字段 SHALL 具有文档注释，禁止使用后置注释。

#### Scenario: 检查文档注释
- **WHEN** 检查 xcell 和 xcell-analyzer 中新增和修改的 public 项
- **THEN** 所有 public 项都有 `///` 风格的文档注释

## MODIFIED Requirements

### Requirement: WorkspaceManager 文件加载
`WorkspaceManager::first_walk()` SHALL 支持通过 glob 模式过滤文件，仅加载匹配指定模式的表格文件。

### Requirement: WorkspaceManager 文件监控
`WorkspaceManager::watcher()` SHALL 在文件变更后自动触发代码生成，并支持防抖和优雅退出。

## REMOVED Requirements

（无移除项）
