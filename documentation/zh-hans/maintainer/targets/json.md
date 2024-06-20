# JSON 数据格式

JSON 是一种轻量级的数据交换格式，XCell 支持将配置数据导出为 JSON 格式。

## 类型映射

| XCell 类型 | JSON 类型 | 说明 |
|-----------|-----------|------|
| `i8` | `number` | 8位有符号整数 |
| `i16` | `number` | 16位有符号整数 |
| `i32` | `number` | 32位有符号整数 |
| `i64` | `number` | 64位有符号整数 |
| `u8` | `number` | 8位无符号整数 |
| `u16` | `number` | 16位无符号整数 |
| `u32` | `number` | 32位无符号整数 |
| `u64` | `number` | 64位无符号整数 |
| `f32` | `number` | 32位浮点数 |
| `f64` | `number` | 64位浮点数 |
| `bool` | `boolean` | 布尔值 |
| `string` | `string` | 字符串 |
| `array<T>` | `array` | 数组 |
| `map<K, V>` | `object` | 映射 |
| `enum` | `string` | 枚举（字符串形式） |
| `struct` | `object` | 结构体 |

## 集成步骤

1. **配置 JSON 导出**：在 XCell 配置中启用 JSON 格式导出
2. **生成 JSON**：使用 XCell 生成 JSON 数据文件
3. **加载 JSON**：在目标平台中加载和解析 JSON 文件
4. **使用数据**：在应用程序中使用解析后的数据

## 示例 JSON

```json
{
  "Player": [
    {
      "id": 1,
      "name": "Player1",
      "level": 10,
      "gold": 1000,
      "is_active": true
    },
    {
      "id": 2,
      "name": "Player2",
      "level": 15,
      "gold": 2000,
      "is_active": true
    }
  ],
  "Item": [
    {
      "id": 1,
      "name": "Sword",
      "damage": 10,
      "price": 100
    }
  ]
}
```

## 注意事项

- JSON 中的数字类型没有精度限制，但在某些语言中可能会有精度问题
- 复杂数据结构会被正确转换为嵌套的 JSON 对象和数组
- 枚举值默认以字符串形式存储，也可以配置为数字形式
- 生成的 JSON 文件可以直接用于前端和后端应用