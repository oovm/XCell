# 合表规则

XCell 支持将多个表格合并为一个表格的功能，通过合表规则可以灵活地管理复杂的数据结构。

## 基本配置

合表规则在 `XCell.toml` 配置文件的 `[merge]` 部分进行配置。

## 配置结构

```toml
[merge]
enable = true

[merge.steps.1]
input = "Item_*.xlsx"
output = "Item.xlsx"

[merge.steps.2]
input = "Monster_*.xlsx"
output = "Monster.xlsx"
```

## 配置项详解

| 配置项 | 类型 | 说明 |
|--------|------|------|
| merge.enable | bool | 是否启用合表功能 |
| merge.steps | map | 合表步骤，key 为步骤序号，value 为合表配置 |

### 合表步骤配置

每个合表步骤包含以下配置：

| 配置项 | 类型 | 说明 |
|--------|------|------|
| input | string | 输入文件匹配模式（支持通配符） |
| output | string | 输出文件名 |

## 合表规则说明

### 执行顺序

合表步骤按照步骤序号从小到大依次执行。

### 文件匹配

- `input` 支持通配符模式
- 例如 `Item_*.xlsx` 会匹配 `Item_Weapon.xlsx`、`Item_Armor.xlsx` 等文件

### 合并逻辑

- 具有相同结构的表格会被合并
- 相同主键的数据会被覆盖（后合并的文件优先级更高）
- 不同的行会被追加到输出表格中

## 示例场景

### 场景1：按类型分表管理

假设您有以下物品表格：
- `Item_Weapon.xlsx` - 武器配置
- `Item_Armor.xlsx` - 防具配置  
- `Item_Consumable.xlsx` - 消耗品配置

可以配置合表规则将它们合并为一个 `Item.xlsx` 表格：

```toml
[merge]
enable = true

[merge.steps.1]
input = "Item_*.xlsx"
output = "Item.xlsx"
```

### 场景2：多语言表格合并

假设您有以下多语言翻译表格：
- `Language_zh-CN.xlsx` - 中文翻译
- `Language_en-US.xlsx` - 英文翻译
- `Language_ja-JP.xlsx` - 日文翻译

可以配置合表规则：

```toml
[merge]
enable = true

[merge.steps.1]
input = "Language_*.xlsx"
output = "Language.xlsx"
```

### 场景3：分步合并

可以配置多个合表步骤，先合并一部分，再合并另一部分：

```toml
[merge]
enable = true

[merge.steps.1]
input = "Basic_*.xlsx"
output = "Basic.xlsx"

[merge.steps.2]
input = "Advanced_*.xlsx"
output = "Advanced.xlsx"

[merge.steps.3]
input = "Basic.xlsx, Advanced.xlsx"
output = "All.xlsx"
```

## 使用说明

1. 在 `XCell.toml` 中配置合表规则
2. 确保 `merge.enable = true`
3. 运行 XCell 工具时会先执行合表操作
4. 合表完成后再进行后续的数据处理和代码生成

## 注意事项

- 合并的表格必须具有相同的结构（相同的列名和类型）
- 如果主键冲突，后合并的文件数据会覆盖先合并的文件数据
- 合表操作会在原始文件所在目录进行
- 建议备份原始文件后再进行合表操作
