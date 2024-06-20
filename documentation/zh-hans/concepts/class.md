# Class 表格类型

Class 表格用于定义全局配置类，适用于单例配置场景。

## 基本结构

- **第一行**：字段注释
- **第二行**：数据类型
- **第三行**：默认值
- **优点**：结构清晰，易于编辑和阅读

### 示例

| field       | comment | type   | default |
| :---------- | ------- | ------ | ------- |
| max\_hp     | 最大生命值   | i32    | 100     |
| user\_name  | 玩家名称    | string | Player  |
| move\_speed | 移动速度    | f32    | 5.0     |

## TOML 配置

Class 表格可以通过 TOML 配置文件来定义元属性：

```toml
# Class 表格示例 - 游戏全局配置
[table]
type = "class"

[fields]
  [fields.max_hp]
type = "int32"
default = 100
meta = "{\"min\": 0, \"max\": 9999}"

  [fields.player_name]
type = "string"
default = "Player"

  [fields.move_speed]
type = "float"
default = 5.0
meta = "{\"min\": 0, \"max\": 20.0}"

  [fields.is_debug_mode]
type = "bool"
default = false
client = true
```

## 使用场景

- 游戏全局配置（如关卡配置、难度配置）
- 应用程序设置
- 系统参数配置

