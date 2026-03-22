# SLG 示例

策略游戏示例，展示科技树和单位系统。

## 表格结构

| 文件 | 类型 | 说明 |
|------|------|------|
| Units.csv | List | 单位表，包含训练建筑引用 |
| Buildings.csv | List | 建筑表，包含科技需求引用 |
| Technologies.csv | List | 科技表，包含前置科技引用列表 |
| UnitUpgrades.csv | List | 单位升级表 |
| Resources.csv | Enum | 资源枚举表 |
| GameSettings.csv | Class | 全局配置 |

## 特性展示

- `ref<TableName>` 外键引用
- `[ref<TableName>]` 引用列表
- Enum 枚举类型
- Class 类型全局配置
- 科技依赖树
- 单位升级系统
