# MMORPG 示例

大型多人在线角色扮演游戏示例，展示复杂的游戏配置系统。

## 表格结构

| 文件 | 类型 | 说明 |
|------|------|------|
| Classes.csv | List | 职业表，包含技能引用列表 |
| Skills.csv | List | 技能表，包含冷却时间类型 |
| Items.csv | List | 物品表，包含品质枚举和颜色类型 |
| Equipment.csv | List | 装备表，包含镶嵌槽和套装引用 |
| Gems.csv | List | 宝石表，属性加成配置 |
| Dungeons.csv | List | 副本表，包含怪物引用 |
| Monsters.csv | List | 怪物表，包含 BOSS 标记 |
| DropTable.csv | List | 掉落表，物品掉落配置 |
| SetBonus.csv | List | 套装效果表 |
| Quests.csv | List | 任务表，包含前置任务引用 |
| Achievements.csv | List | 成就表 |
| Events.csv | List | 活动表，包含 datetime 时间类型 |
| Quality.csv | Enum | 品质枚举表 |
| GuildLevels.csv | Class | 公会等级配置（全局配置） |

## 特性展示

- `datetime` 日期时间类型
- `time` 时间类型
- `color` 颜色类型
- `&TableName` 外键引用
- `[&TableName]` 引用列表
- Class 类型全局配置
- Enum 枚举类型
