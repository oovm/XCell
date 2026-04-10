# Tasks

- [x] Task 1: 修复 xcell-config Unity 配置模块
  - [x] SubTask 1.1: 为 `UnityStorage`、`UnityCodegen`、`UnityBinaryConfig`、`UnityJsonConfig`、`UnityXmlConfig`、`UnityProtobufConfig`、`UnityXluaConfig`、`UnityCodegenConfig` 的所有 public 字段添加文档注释
  - [x] SubTask 1.2: 为 `UnityStorage` 和 `UnityCodegen` 的所有 public 方法添加文档注释
  - [x] SubTask 1.3: 修复 `project/der.rs` 中 Unity 反序列化，使其能正确从 TOML 的 `[[generators]]` 格式传递 `storage` 和 `xlua` 子配置
  - [x] SubTask 1.4: 添加 `UnityCodegen::validate()` 方法，验证配置的合法性（output 非空、namespace 合法等）

- [x] Task 2: 修复 xcell-generator Unity 代码生成模块
  - [x] SubTask 2.1: 在 `codegen/mod.rs` 中取消 `pub mod unity;` 的注释
  - [x] SubTask 2.2: 在 `lib.rs` 中取消 Unity 生成器注册的注释
  - [x] SubTask 2.3: 修复 `unity/language.rs`：将 `UnityLanguage` 改为 `UnityLanguageTemplate`，修复模板路径为 `"BuildLanguage.cs.dejavu"`，添加缺失的 `BuildLanguage.cs.dejavu` 模板文件
  - [x] SubTask 2.4: 修复 `unity/dictionary.rs`：将 `ws.config.unity.loader.suffix_table` 等引用改为使用 `self`（即 `UnityCodegen` 自身）的字段，统一路径计算逻辑
  - [x] SubTask 2.5: 修复 `unity/manager.rs`：从 workspace 数据填充 `tables` 向量，包含所有 class/dict/list 表信息
  - [x] SubTask 2.6: 完善 `unity/binary.rs`：取消注释并修复 `write_language_keys`、`write_language_tables`、`write_language_table` 方法，使其与当前 WorkspaceManager API 兼容
  - [x] SubTask 2.7: 修复 `unity/mod.rs` 中 `write_csharp` 方法，添加对 dict/list/language/binary 的调用
  - [x] SubTask 2.8: 为所有 Unity 代码生成模块的 public 结构体、枚举、方法、字段添加文档注释

- [x] Task 3: 修复 Unity 代码生成测试
  - [x] SubTask 3.1: 修复 `tests/unity/mod.rs` 中 `UnityCodegen::new()` 调用（`UnityCodegen` 来自 xcell_config，无 `new()` 方法，应使用 `Default::default()`）
  - [x] SubTask 3.2: 添加带 WorkspaceManager 的集成测试，验证 class/enumerate/dictionary/manager 代码生成
  - [x] SubTask 3.3: 添加配置反序列化测试，验证从 TOML 正确解析 Unity 配置

- [x] Task 4: 创建缺失的模板文件
  - [x] SubTask 4.1: 创建 `templates/BuildLanguage.cs.dejavu` 模板文件

- [ ] Task 5: 编译验证
  - [x] SubTask 5.1: 运行 `cargo check -p xcell-config` 确保编译通过
  - [x] SubTask 5.2: 运行 `cargo check -p xcell-generator` 确保编译通过
  - [ ] SubTask 5.3: 运行 `cargo test -p xcell-generator` 确保测试通过

# Task Dependencies
- [Task 1] 是 [Task 2] 的前置依赖（config 修复后 generator 才能正确编译）
- [Task 4] 是 [Task 2.3] 的前置依赖（模板文件需要先存在）
- [Task 2] 是 [Task 3] 的前置依赖（模块修复后才能编写有效测试）
- [Task 5] 是所有其他 Task 的最终验证步骤
