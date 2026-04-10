# 完善 Unity 代码生成与配置管理 Spec

## Why
Unity 代码生成模块当前处于禁用状态（`codegen/mod.rs` 和 `lib.rs` 中被注释），且存在多处编译错误和逻辑缺陷。xcell-config 中 Unity 配置的反序列化路径与 ProjectConfig.toml 实际格式不匹配，导致 Unity 生成器无法正常工作。需要修复这些问题以推进 roadmap 中代码生成组的工作。

## What Changes
- **重新启用 Unity 代码生成模块**：取消 `codegen/mod.rs` 和 `lib.rs` 中的注释，修复编译错误
- **修复 Unity 代码生成逻辑**：
  - 修复 `language.rs` 中引用不存在的 `UnityLanguage` 类型（应为 `UnityLanguageTemplate`）
  - 修复 `dictionary.rs` 中引用 `ws.config.unity.loader` 的路径错误
  - 修复 `manager.rs` 中 `tables` 向量为空的问题，从 workspace 数据填充
  - 完善 `binary.rs` 中被注释掉的语言表二进制生成方法
- **修复 xcell-config Unity 配置**：
  - 修复 `project/der.rs` 中 Unity 反序列化未传递 `storage`/`xlua` 配置的问题
  - 统一 `UnityCodegen` 在 xcell-config 和 xcell-generator 中的使用方式
- **添加文档注释**：为所有 public 的结构体、枚举、方法、字段添加文档注释
- **完善配置管理系统**：增加 Unity 配置验证逻辑
- **建立 Unity 代码生成测试套件**：编写端到端测试验证 Unity C# 代码生成

## Impact
- Affected specs: xcell-generator, xcell-config
- Affected code:
  - `backends/xcell-generator/src/codegen/mod.rs`
  - `backends/xcell-generator/src/codegen/unity/mod.rs`
  - `backends/xcell-generator/src/codegen/unity/class.rs`
  - `backends/xcell-generator/src/codegen/unity/dictionary.rs`
  - `backends/xcell-generator/src/codegen/unity/enumerate.rs`
  - `backends/xcell-generator/src/codegen/unity/language.rs`
  - `backends/xcell-generator/src/codegen/unity/manager.rs`
  - `backends/xcell-generator/src/codegen/unity/binary.rs`
  - `backends/xcell-generator/src/lib.rs`
  - `backends/xcell-config/src/unity/mod.rs`
  - `backends/xcell-config/src/unity/der.rs`
  - `backends/xcell-config/src/unity/ser.rs`
  - `backends/xcell-config/src/project/der.rs`
  - `backends/xcell-generator/tests/unity/mod.rs`

## ADDED Requirements

### Requirement: Unity 代码生成模块重新启用
系统 SHALL 在 `xcell-generator` 中重新启用 Unity 代码生成模块，使其能够被正常注册和调用。

#### Scenario: Unity 生成器注册成功
- **WHEN** 创建 `Generator` 实例
- **THEN** Unity 生成器被注册到 generators 映射中，`has_generator("unity")` 返回 `true`

#### Scenario: Unity 生成器可被调用
- **WHEN** 配置中启用了 Unity 产物
- **THEN** Generator 能找到并调用 Unity 代码生成器

### Requirement: Unity C# 类代码生成
系统 SHALL 能根据 WorkspaceManager 中的 XClassData 生成正确的 C# 类代码文件。

#### Scenario: 生成类代码文件
- **WHEN** WorkspaceManager 包含类表数据
- **THEN** 在配置的输出目录下生成对应的 `.cs` 文件，包含正确的命名空间、字段声明和默认值

### Requirement: Unity C# 枚举代码生成
系统 SHALL 能根据 XEnumerateData 生成正确的 C# 枚举代码文件。

#### Scenario: 生成枚举代码文件
- **WHEN** WorkspaceManager 包含枚举表数据
- **THEN** 在配置的输出目录下生成对应的 `.cs` 枚举文件，包含枚举定义和扩展方法

### Requirement: Unity C# 字典代码生成
系统 SHALL 能根据 XDictData 和 XListData 生成正确的 C# 字典代码文件。

#### Scenario: 生成字典代码文件
- **WHEN** WorkspaceManager 包含字典表数据
- **THEN** 在配置的输出目录下生成对应的 `.cs` 字典文件，实现 `IReadOnlyDictionary` 接口和二进制读写

### Requirement: Unity C# 管理器代码生成
系统 SHALL 能根据 WorkspaceManager 中的所有表数据生成 DataTableManager.cs 管理器文件。

#### Scenario: 生成管理器代码文件
- **WHEN** WorkspaceManager 包含表数据
- **THEN** 生成 `DataTableManager.cs`，包含所有表的懒加载单例和 Reload/Clear 方法

### Requirement: Unity 二进制数据生成
系统 SHALL 能生成 Unity 可读取的二进制 `.bytes` 数据文件。

#### Scenario: 生成二进制数据文件
- **WHEN** Unity 配置中 `storage.binary.enable` 为 `true`
- **THEN** 在配置的二进制输出目录下生成 `.bytes` 文件

### Requirement: Unity 配置反序列化修复
系统 SHALL 能正确从 ProjectConfig.toml 反序列化 Unity 配置，包括 storage 和 xlua 子配置。

#### Scenario: 从 TOML 解析完整 Unity 配置
- **WHEN** ProjectConfig.toml 包含 Unity 生成器配置及 storage 子表
- **THEN** `UnityCodegen` 的 `storage` 和 `xlua` 字段被正确填充

### Requirement: 文档注释完整性
所有 public 的结构体、枚举、方法、字段 SHALL 具有文档注释，禁止使用后置注释。

#### Scenario: 检查文档注释
- **WHEN** 检查 xcell-generator 和 xcell-config 中 Unity 相关代码
- **THEN** 所有 public 项都有 `///` 风格的文档注释

### Requirement: Unity 代码生成测试套件
系统 SHALL 提供完整的 Unity 代码生成测试，验证各类型代码文件的生成正确性。

#### Scenario: 测试通过
- **WHEN** 运行 `cargo test -p xcell-generator`
- **THEN** 所有 Unity 相关测试通过

## MODIFIED Requirements

### Requirement: Generator 生成器注册
`Generator::new()` SHALL 注册 Unity 代码生成器，使 `generator_count()` 返回值增加 1。

## REMOVED Requirements

（无移除项）
