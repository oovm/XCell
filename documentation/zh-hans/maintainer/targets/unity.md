# Unity 集成

XCell 提供了与 Unity 引擎的深度集成，支持生成 C# 代码、二进制数据文件等多种格式。

## 配置选项

在 `XCell.toml` 文件中，Unity 集成配置位于 `[unity]` 部分：

```toml
[unity]
enable = true
project = "../"
output = "Assets/Scripts/DataTable/Generated"
namespace = "DataTable.Generated"
manager = "DataTableManager"
suffix_table = "Table"
suffix_element = "Element"
support_clone = true
legacy_using = false
legacy_null_null = false

[unity.binary]
enable = true
output = "Assets/Tables/Generated"

[unity.xlua]
enable = false

[unity.xml]
enable = false
output = "Assets/Tables/Readable"

[unity.json]
enable = false
output = "Assets/Tables/Readable"

[unity.protobuf]
enable = false
```

## 生成的代码结构

### 表类结构

每个配置表会生成对应的 C# 类，包含：
- 表数据类（Table）
- 元素数据类（Element）
- 管理器类（Manager）

### 示例生成代码结构：

```csharp
namespace DataTable.Generated
{
    public class BuffTable
    {
        public Dictionary<int, BuffElement> Data { get; }
        public BuffElement Get(int id);
        public bool TryGet(int id, out BuffElement element);
    }

    public class BuffElement
    {
        public int Id { get; }
        public string Name { get; }
        public int Value { get; }
        // ... 其他字段
    }

    public class DataTableManager
    {
        public BuffTable BuffTable { get; }
        public void LoadAll();
        public void UnloadAll();
    }
}
```

## 二进制数据加载

### 加载二进制数据：

```csharp
var manager = new DataTableManager();
manager.LoadAll();
```

### 支持的功能：
- 异步加载
- 增量加载
- 内存管理
- 热更新支持

## 类型映射

XCell 类型到 C# 类型映射：

| XCell 类型 | C# 类型 |
|------------|---------|
| bool | bool |
| i8 | sbyte |
| i16 | short |
| i32 | int |
| i64 | long |
| u8 | byte |
| u16 | ushort |
| u32 | uint |
| u64 | ulong |
| f32 | float |
| f64 | double |
| string | string |
| color | UnityEngine.Color |
| vec2 | UnityEngine.Vector2 |
| vec3 | UnityEngine.Vector3 |
| vec4 | UnityEngine.Vector4 |
| quaternion | UnityEngine.Quaternion |

## XLua 集成

启用 XLua 支持：

```toml
[unity.xlua]
enable = true
```

XLua 集成提供：
- Lua 绑定代码生成
- 高性能 Lua 访问接口
- 类型安全的 Lua API

## 性能优化

### 大表处理优化

1. **合理拆分大表**
   - 将大表按功能或模块拆分
   - 使用行合并规则在构建时合并
   - 保持开发时的可维护性和运行时的性能

2. **使用二进制格式**
   - 二进制格式加载速度最快
   - 生产环境推荐使用二进制格式
   - 开发环境可以使用 XML/JSON 便于调试

### 增量更新优化

1. **监听模式**
   使用监听模式：
   ```bash
   xcell.exe --watch
   ```
   监听模式特点：
   - 只重新生成变更的文件
   - 大幅提升开发效率
   - 支持实时预览

2. **合理配置监听**
   配置合理的 include/exclude 模式，减少监听文件数量。

### 内存优化

1. **只加载需要的表**
   ```csharp
   // 只加载特定的表
   manager.BuffTable.Load();
   manager.ItemTable.Load();
   ```

2. **及时卸载**
   ```csharp
   // 卸载不需要的表
   manager.BuffTable.Unload();
   ```

## 常见问题

### 生成的代码编译错误

- 检查 Unity 项目路径是否正确
- 检查命名空间是否与项目结构匹配
- 确保所有依赖项已正确安装

### 数据加载失败

- 检查二进制文件是否已生成
- 检查文件路径是否正确
- 确保表结构与数据类型匹配

## 最佳实践

1. **使用元表格式**：对于复杂的配置表，推荐使用元表格式，支持更丰富的元数据定义
2. **合理使用合表规则**：对于大型项目，使用合表规则管理复杂表格
3. **优化数据结构**：根据实际使用场景选择合适的数据类型和结构
4. **定期清理**：定期清理不需要的配置表和数据，保持项目整洁

## 示例项目

XCell 提供了 Unity 示例项目，展示了如何在实际项目中使用 XCell：

- 基本配置表使用
- 复杂数据结构
- 多语言支持
- 热更新集成

通过示例项目，您可以快速了解 XCell 在 Unity 中的最佳实践。