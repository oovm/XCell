# Dictionary 表格类型

Dictionary 表格用于存储键值对形式的数据集合，XCell 支持两种 Dictionary 表格：List（列表）和 Dict（字典）。

## List 表格

List 表格以整数 ID 作为主键，适用于需要按顺序访问的数据。

### 基本结构

- **直接定义字段**：第一行开始直接定义字段名和数据
- **优点**：结构紧凑，易于编辑和阅读

### 示例

| id | monster_name | hit_points | attack_damage |
|----|-------------|------------|---------------|
| 1  | 史莱姆      | 50         | 5             |
| 2  | 哥布林      | 80         | 10            |
| 3  | 骷髅兵      | 100        | 15            |

### 代码生成示例

```csharp
public class MonsterTable
{
    public Dictionary<int, MonsterElement> Data { get; } = new Dictionary<int, MonsterElement>();
    
    public MonsterElement Get(int id) => Data[id];
    public bool TryGet(int id, out MonsterElement element) => Data.TryGetValue(id, out element);
}

public class MonsterElement
{
    /// <summary>
    /// 主键
    /// </summary>
    public int Id { get; set; }
    
    /// <summary>
    /// 怪物名称
    /// </summary>
    public string MonsterName { get; set; }
    
    /// <summary>
    /// 生命值
    /// </summary>
    public int HitPoints { get; set; }
    
    /// <summary>
    /// 攻击力
    /// </summary>
    public int AttackDamage { get; set; }
}
```

## Dict 表格

Dict 表格以字符串作为主键，适用于需要按名称访问的数据。

### 基本结构

- **直接定义字段**：第一行开始直接定义字段名和数据
- **优点**：结构紧凑，易于编辑和阅读

### 示例

| quality_key | item_icon | display_color |
|-------------|-----------|---------------|
| common      | item_01.png | #FFFFFF      |
| rare        | item_02.png | #00FFFF      |
| epic        | item_03.png | #FF00FF      |

### 代码生成示例

```csharp
public class ItemQualityTable
{
    public Dictionary<string, ItemQualityElement> Data { get; } = new Dictionary<string, ItemQualityElement>();
    
    public ItemQualityElement Get(string key) => Data[key];
    public bool TryGet(string key, out ItemQualityElement element) => Data.TryGetValue(key, out element);
}

public class ItemQualityElement
{
    /// <summary>
    /// 品质键
    /// </summary>
    public string QualityKey { get; set; }
    
    /// <summary>
    /// 物品图标
    /// </summary>
    public string ItemIcon { get; set; }
    
    /// <summary>
    /// 显示颜色
    /// </summary>
    public Color DisplayColor { get; set; }
}
```

## TOML 配置

Dictionary 表格可以通过 TOML 配置文件来定义元属性：

### List 表格配置

```toml
# List 表格示例 - 怪物配置
[table]
type = "table"

[fields]
  [fields.id]
  type = "int"
  
  [fields.name]
  type = "string"
  
  [fields.hit_points]
type = "int"
meta = "{\"min\": 1}"
  
  [fields.attack_damage]
type = "int"
meta = "{\"min\": 1}"
  
  [fields.item_drop_id]
type = "int"
ref = "ItemTable"
```

### Dict 表格配置

```toml
# Dict 表格示例 - 物品品质配置
[table]
type = "table"

[fields]
  [fields.quality_key]
  type = "string"
  
  [fields.item_icon]
  type = "string"
  
  [fields.display_color]
  type = "string"
```

## 使用场景

- **List 表格**：怪物配置、道具配置、技能配置等需要顺序或按数字ID访问的数据
- **Dict 表格**：品质配置、UI 配置、音效配置等需要按名称访问的数据
