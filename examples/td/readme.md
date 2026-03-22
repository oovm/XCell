# Tower Defense 示例

塔防游戏示例，展示塔升级和波次系统。

## 表格结构

| 文件 | 类型 | 说明 |
|------|------|------|
| Towers.csv | List | 塔表，包含自引用升级和 vec3 范围 |
| Enemies.csv | List | 敌人表 |
| Waves.csv | List | 波次表，包含敌人数量元组列表 |
| Maps.csv | List | 地图表，包含路径配置 |
| Projectiles.csv | List | 弹道表 |
| Upgrades.csv | List | 升级表 |
| Effects.csv | List | 效果表 |
| GameSettings.csv | Class | 全局配置 |

## 特性展示

- `vec3<f32>` 3D 向量类型
- `[(&Enemies, i32)]` 元组列表（敌人+数量）
- `&Towers` 自引用（塔升级）
- `&TableName` 外键引用
- Class 类型全局配置
- 波次配置系统
