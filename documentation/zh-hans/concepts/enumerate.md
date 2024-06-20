# Enumerate 表格类型

Enumerate 表格用于定义枚举类型，同时还可以为每个枚举值附加额外的数据。

## 基本结构

- **第一行**：字段名
- **第二行**：数据类型
- **第三行及以后**：数据行
- **优点**：结构清晰，易于编辑和阅读

### 示例

| 字段    | 注释  | comment | icon         |
| :---- | --- | ------- | ------------ |
| name  | id  | comment | icon         |
| enum  | i32 | utf8    | utf8         |
| Norma | 1   | 普通品质    | icon\_01.png |
| Rare  | 2   | 稀有品质    | icon\_02.png |
| Epic  | 3   | 史诗品质    | icon\_03.png |
| Super | 4   | 传说品质    | icon\_04.png |

## TOML 配置

Enumerate 表格可以通过 TOML 配置文件来定义元属性：

```toml
# Enumerate 表格示例 - 物品品质
[table]
type = "enum"

[fields]
  [fields.id]
  type = "int"
  
  [fields.text]
  type = "string"
  
  [fields.icon]
  type = "string"
  
  [fields.color]
  type = "string"
```

## 使用场景

- 物品品质定义
- 角色状态定义
- 事件类型定义
- 任何需要枚举且需要附加额外数据的场景

