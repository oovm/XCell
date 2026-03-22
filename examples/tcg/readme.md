# TCG 示例

集换式卡牌游戏示例，展示卡牌效果和卡组系统。

## 表格结构

| 文件 | 类型 | 说明 |
|------|------|------|
| Cards.csv | List | 卡牌表，包含颜色类型和效果引用 |
| Classes.csv | List | 职业表 |
| Effects.csv | List | 效果表 |
| Heroes.csv | List | 英雄表 |
| Decks.csv | List | 卡组表，包含卡牌数量元组 |
| CardSets.csv | List | 卡包表 |
| Rarity.csv | Enum | 稀有度枚举表 |
| Keywords.csv | Enum | 关键词枚举表 |
| GameSettings.csv | Class | 全局配置 |

## 特性展示

- `color` 颜色类型
- `[(&Cards, i32)]` 元组列表（卡牌+数量）
- `[&TableName]` 引用列表
- Enum 枚举类型
- Class 类型全局配置
- 卡组配置系统
