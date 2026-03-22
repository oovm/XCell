# Galgame 示例

视觉小说/恋爱游戏示例，展示分支剧情和好感度系统。

## 表格结构

| 文件 | 类型 | 说明 |
|------|------|------|
| Characters.csv | List | 角色表，包含角色基本信息和唯一名称约束 |
| Scenes.csv | List | 场景表，定义游戏场景和背景音乐 |
| Dialogues.csv | List | 对话表，包含角色引用和场景引用 |
| Choices.csv | List | 选项表，分支选项和好感度变化 |
| Affection.csv | List | 好感度等级表，定义好感等级和解锁内容 |
| AffectionCondition.csv | List | 好感度条件表，触发特定剧情的条件 |
| Achievements.csv | List | 成就表，游戏成就定义 |
| Endings.csv | List | 结局表，多种结局配置 |

## 特性展示

- `@text` 唯一名称约束
- `ref<TableName>` 外键引用
- `[ref<TableName>]` 引用列表
- 分支剧情系统
- 好感度系统
