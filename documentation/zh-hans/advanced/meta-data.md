# 元属性（Meta Data）高级特性

本文档介绍 XCell 中元属性的高级特性和最佳实践，帮助您更灵活地使用元数据来增强表格功能。

## 新的配置方式

XCell 现在采用非侵入式的配置方式，将所有元属性和复杂属性都存储在 TOML 配置文件中，而不是表格文件中。这种方式使表格文件更加简洁，只包含数据，提高了表格的可读性和可维护性。

### 配置文件格式

```toml
[line]
typing = 3
field = 2
helper = 1
data = 4

[typing]
# 类型元信息配置...

[unity]
# Unity 代码生成配置...

[[fields]]
name = "id"
type = "int"
comment = "唯一标识符"

[[fields]]
name = "name"
type = "string"
comment = "名称"

[[fields]]
name = "value"
type = "float"
comment = "数值"
default = "0.0"
```

## 使用 toml edit 命令编辑配置文件

XCell 提供了 `toml` 命令来编辑 TOML 配置文件，支持添加、删除、更新和列出字段配置。

### 列出字段配置

```bash
xcell toml list --file path/to/config.toml
```

### 添加字段配置

```bash
xcell toml add --file path/to/config.toml --name field_name --type field_type --comment "字段注释" --default "默认值"
```

### 删除字段配置

```bash
xcell toml remove --file path/to/config.toml --name field_name
```

### 更新字段配置

```bash
xcell toml update --file path/to/config.toml --name field_name --type new_type --comment "新注释" --default "新默认值"
```

## 元数据的高级用法

### 字段验证规则

元数据可以用于定义字段的验证规则，确保数据的有效性：

```toml
[[fields]]
name = "age"
type = "int"
comment = "年龄"
meta = "{\"min\": 1, \"max\": 100}"

[[fields]]
name = "email"
type = "string"
comment = "邮箱"
meta = "{\"pattern\": \"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$\"}"
```

### 条件逻辑

元数据可以包含条件逻辑，根据其他字段的值来动态调整字段的行为：

```toml
[[fields]]
name = "is_vip"
type = "bool"
comment = "是否VIP"
default = "false"

[[fields]]
name = "vip_level"
type = "int"
comment = "VIP等级"
default = "0"
meta = "{\"condition\": \"is_vip == true\", \"min\": 1, \"max\": 10}"
```

### 复杂数据结构

元数据可以定义复杂的数据结构，如数组、对象等：

```toml
[[fields]]
name = "skills"
type = "array"
comment = "技能列表"
meta = "{\"item_type\": \"string\", \"max_length\": 5}"

[[fields]]
name = "stats"
type = "object"
comment = "属性"
meta = "{\"properties\": {\"strength\": {\"type\": \"int\", \"min\": 1}, \"agility\": {\"type\": \"int\", \"min\": 1}}}"
```

## 元数据与代码生成

元数据可以影响代码生成过程，为生成的代码添加额外的特性：

### 自定义注释

```toml
[[fields]]
name = "player_name"
type = "string"
comment = "玩家的显示名称"
meta = "{\"max_length\": 20}"
```

生成的代码：

```csharp
/// <summary>
/// 玩家的显示名称
/// </summary>
public string PlayerName { get; set; }
```

### 序列化选项

```toml
[[fields]]
name = "score"
type = "int"
comment = "分数"
meta = "{\"serialize\": true, \"json_name\": \"player_score\"}"
```

生成的代码：

```csharp
[JsonProperty("player_score")]
public int Score { get; set; }
```

### 验证属性

```toml
[[fields]]
name = "age"
type = "int"
comment = "年龄"
meta = "{\"min\": 1, \"max\": 100, \"required\": true}"
```

生成的代码：

```csharp
[Range(1, 100)]
[Required]
public int Age { get; set; }
```

## 迁移现有表格文件

### 自动迁移工具

XCell 提供了自动迁移工具，可以从现有表格文件中提取元表信息，并生成对应的 TOML 配置文件。

### 手动迁移步骤

1. 创建与表格文件同名的 TOML 配置文件（例如，`Player.xlsx` 对应 `Player.toml`）
2. 在 TOML 配置文件中定义字段配置信息
3. 从表格文件中移除元表设定，只保留数据
4. 运行 XCell 验证配置是否正确

## 元数据的最佳实践

### 命名规范

- 使用驼峰命名法命名字段
- 使用 Pascal 命名法命名类型
- 保持元数据键名的一致性

### 性能优化

- 只在必要时使用复杂的元数据
- 避免在元数据中存储大量数据
- 使用缓存来减少元数据解析的开销

### 可维护性

- 将相关的元数据组织在一起
- 使用注释来解释复杂的元数据
- 定期清理未使用的元数据

## 高级元数据示例

### 完整的角色配置

```toml
[line]
typing = 3
field = 2
helper = 1
data = 4

[[fields]]
name = "id"
type = "int"
comment = "角色唯一ID"
meta = "{\"min\": 1}"

[[fields]]
name = "name"
type = "string"
comment = "角色名称"
meta = "{\"max_length\": 30}"

[[fields]]
name = "level"
type = "int"
comment = "角色等级"
default = "1"
meta = "{\"min\": 1, \"max\": 100}"

[[fields]]
name = "stats"
type = "object"
comment = "属性"
meta = "{\"properties\": {
  \"strength\": {\"type\": \"int\", \"min\": 1, \"comment\": \"力量\"},
  \"agility\": {\"type\": \"int\", \"min\": 1, \"comment\": \"敏捷\"},
  \"intelligence\": {\"type\": \"int\", \"min\": 1, \"comment\": \"智力\"}
}}"

[[fields]]
name = "skills"
type = "array"
comment = "角色技能"
meta = "{\"item_type\": \"string\", \"max_length\": 5}"

[[fields]]
name = "equipment"
type = "object"
comment = "装备"
meta = "{\"properties\": {
  \"weapon\": {\"type\": \"string\", \"comment\": \"武器\"},
  \"armor\": {\"type\": \"string\", \"comment\": \" armor\"},
  \"accessory\": {\"type\": \"string\", \"comment\": \"饰品\"}
}}"
```

### 多语言支持配置

```toml
[line]
typing = 3
field = 2
helper = 1
data = 4

[[fields]]
name = "key"
type = "string"
comment = "语言键"

[[fields]]
name = "group"
type = "string"
comment = "语言分组"

[[fields]]
name = "zh_cn"
type = "language"
comment = "简体中文"

[[fields]]
name = "en_us"
type = "language"
comment = "English (US)"

[[fields]]
name = "ja_jp"
type = "language"
comment = "日本語"

[[fields]]
name = "ko_kr"
type = "language"
comment = "한국어"
```

## 总结

元数据是 XCell 中非常强大的功能，通过合理使用元数据，您可以：

1. **增强数据验证**：确保数据的有效性和一致性
2. **自定义代码生成**：生成符合特定需求的代码
3. **实现复杂逻辑**：支持条件判断和复杂数据结构
4. **提高可维护性**：通过 TOML 配置文件来组织配置

通过本文档介绍的高级特性和最佳实践，您可以充分发挥元数据的潜力，为您的项目创建更加灵活、强大的配置系统。