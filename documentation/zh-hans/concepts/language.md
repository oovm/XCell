# Language 表格类型

Language 表格用于管理多语言文本，XCell 提供了完整的国际化支持。

## 基本结构

- **第一行**：字段名
- **第二行**：数据类型
- **第三行及以后**：数据行
- **优点**：结构清晰，易于编辑和阅读

### 示例

| 语言键名称        | 语言分组  | 中文翻译     | 英文翻译     | 日文翻译     |
| ------------ | ----- | -------- | -------- | -------- |
| key          | group | zh\_cn   | en\_us   | ja\_jp   |
| utf8         | utf8  | language | language | language |
| Ui\_Start    | ui    | 开始       | Start    | スタート     |
| Ui\_Settings | ui    | 设置       | Settings | 設定       |
| Ui\_Exit     | ui    | 退出       | Exit     | 終了       |

## TOML 配置

Language 表格可以通过 TOML 配置文件来定义元属性：

```toml
# Language 表格示例 - 多语言支持
[table]
type = "language"

[fields]
  [fields.key]
  type = "string"
  
  [fields.group]
  type = "string"
  
  [fields.zh_cn]
  type = "language"
  
  [fields.en_us]
  type = "language"
  
  [fields.ja_jp]
  type = "language"
```

## 使用场景

- 游戏 UI 多语言支持
- 应用程序国际化
- 多语言文档系统
- 需要支持多种语言的任何项目

