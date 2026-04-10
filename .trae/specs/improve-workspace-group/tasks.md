# Tasks

- [ ] Task 1: 修复 CLI 描述文档和移除调试输出
  - [ ] SubTask 1.1: 修复 `XCellArgs` 结构体的 `#[command(about)]` 描述，从 "Simple program to greet a person" 改为 "XCell 配置表管理工具"
  - [ ] SubTask 1.2: 将 `WorkspaceManager::try_perform_file` 中的 `println!` 替换为 `tracing::debug!`/`tracing::info!`
  - [ ] SubTask 1.3: 将 `main.rs` 中的 `println!` 调试输出替换为 `tracing::info!`/`tracing::debug!`

- [ ] Task 2: 完善 WorkspaceManager 工作空间管理方法
  - [ ] SubTask 2.1: 在 `xcell-analyzer/src/config/mod.rs` 中添加 `WorkspaceStatus` 结构体，包含表数量、枚举数量、字典数量、类数量等字段
  - [ ] SubTask 2.2: 为 `WorkspaceManager` 实现 `status()` 方法，返回 `WorkspaceStatus`
  - [ ] SubTask 2.3: 为 `WorkspaceManager` 实现 `summary()` 方法，返回格式化的摘要字符串

- [ ] Task 3: 增加 CLI 日志级别控制和命令行选项
  - [ ] SubTask 3.1: 在 `XCellArgs` 中添加 `--verbose` 和 `--quiet` 选项
  - [ ] SubTask 3.2: 在 `XCellArgs` 中添加 `--dry-run` 选项
  - [ ] SubTask 3.3: 在 `XCellArgs` 中添加 `--filter` 选项（glob 模式过滤表格）
  - [ ] SubTask 3.4: 修改 `logger()` 函数，根据 `--verbose`/`--quiet` 选项设置日志级别

- [ ] Task 4: 增加 CLI 子命令
  - [ ] SubTask 4.1: 添加 `Info` 子命令，输出工作空间配置和状态摘要
  - [ ] SubTask 4.2: 添加 `Init` 子命令，在指定目录创建默认 `ProjectConfig.toml`
  - [ ] SubTask 4.3: 添加 `List` 子命令，列出工作空间中已加载的表格及其类型

- [ ] Task 5: 优化文件格式检测性能
  - [ ] SubTask 5.1: 修改 `FileFormatDetector::detect_by_content`，仅读取文件头部最多 8KB 内容进行判断，避免全量读取

- [ ] Task 6: 完善文件监控功能
  - [ ] SubTask 6.1: 在 `watcher()` 中处理文件创建事件，自动加载新文件
  - [ ] SubTask 6.2: 在 `watcher()` 中处理文件删除事件，从工作空间移除对应数据
  - [ ] SubTask 6.3: 为 `watcher()` 实现防抖机制，使用 tokio::time::sleep 延迟处理
  - [ ] SubTask 6.4: 文件变更后自动触发代码生成
  - [ ] SubTask 6.5: 支持 Ctrl+C 优雅退出监控模式

- [ ] Task 7: 支持 first_walk 中的表格过滤
  - [ ] SubTask 7.1: 为 `first_walk()` 添加可选的 glob 过滤参数，仅加载匹配模式的文件
  - [ ] SubTask 7.2: 在 `main.rs` 中将 `--filter` 选项传递给 `first_walk()`

- [ ] Task 8: 添加文档注释
  - [ ] SubTask 8.1: 为 `WorkspaceStatus` 结构体及其字段添加文档注释
  - [ ] SubTask 8.2: 为新增的 `WorkspaceManager` 方法添加文档注释
  - [ ] SubTask 8.3: 为新增的 CLI 子命令和选项添加文档注释
  - [ ] SubTask 8.4: 为 `FileFormatDetector` 修改的方法补充文档注释

# Task Dependencies
- [Task 2] depends on [Task 1] (先清理调试输出再添加新方法)
- [Task 3] depends on [Task 1] (先修复 CLI 描述再添加新选项)
- [Task 4] depends on [Task 2] (info 子命令需要 status 方法)
- [Task 4] depends on [Task 3] (子命令需要日志级别控制)
- [Task 7] depends on [Task 3] (filter 选项需要在 CLI 中先定义)
- [Task 6] depends on [Task 1] (先清理调试输出再完善监控)
