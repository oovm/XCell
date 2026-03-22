# RPG 示例

角色扮演游戏示例，展示装备系统和任务系统。

## 表格结构

| 文件 | 类型 | 说明 |
|------|------|------|
| Skills.csv | List | 技能表，包含物品引用和时间类型 |
| Item.csv | List | 物品表，包含品质枚举和颜色类型 |
| Monsters.csv | List | 怪物表，包含类型引用和技能引用列表 |
| PlayerLevels.csv | List | 玩家等级表 |
| Equipment.csv | List | 装备表，包含宝石镶嵌槽 |
| Quest.csv | List | 任务表，包含前置任务引用 |
| Dungeon.csv | List | 副本表，包含怪物引用 |
| DropTable.csv | List | 掉落表，物品掉落配置 |
| Gem.csv | List | 宝石表 |
| SetBonus.csv | List | 套装效果表 |
| Quality.csv | Enum | 品质枚举表 |
| MonsterType.csv | Enum | 怪物类型枚举表 |

## 特性展示

- `&TableName` 外键引用
- `[&TableName]` 引用列表
- `time` 时间类型
- `color` 颜色类型
- Enum 枚举类型
- 自引用（任务前置任务）
- 套装系统
