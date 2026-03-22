# Roguelike 示例

Roguelike 游戏示例，展示随机事件和遗物系统。

## 表格结构

| 文件 | 类型 | 说明 |
|------|------|------|
| Characters.csv | List | 角色表，包含初始遗物引用 |
| Cards.csv | List | 卡牌表，卡牌战斗系统 |
| Effects.csv | List | 效果表，各种游戏效果 |
| Enemies.csv | List | 敌人表，包含精英和 BOSS |
| Events.csv | List | 随机事件表 |
| EventOptions.csv | List | 事件选项表，包含事件链引用 |
| Relics.csv | List | 遗物表，包含稀有度枚举引用 |
| Rooms.csv | List | 房间表，包含 vec2 位置类型 |
| Rarity.csv | Enum | 稀有度枚举表 |
| GameSettings.csv | Class | 全局配置 |

## 特性展示

- `vec2<i32>` 2D 向量类型
- `ref<TableName>` 外键引用
- `[ref<TableName>]` 引用列表
- Enum 枚举引用
- Class 类型全局配置
- 事件链系统
- 遗物系统
