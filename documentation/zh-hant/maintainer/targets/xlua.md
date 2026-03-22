# XLua 集成

XLua 是 Unity 中常用的 Lua 指令碼解決方案，XCell 提供了與 XLua 的集成支援。

## 類型對應

| XCell 類型 | XLua 類型 | 说明 |
|-----------|-----------|------|
| `i8` | `number` | 8位有符號整数 |
| `i16` | `number` | 16位有符號整数 |
| `i32` | `number` | 32位有符號整数 |
| `i64` | `number` | 64位有符號整数 |
| `u8` | `number` | 8位无符號整数 |
| `u16` | `number` | 16位无符號整数 |
| `u32` | `number` | 32位无符號整数 |
| `u64` | `number` | 64位无符號整数 |
| `f32` | `number` | 32位浮点数 |
| `f64` | `number` | 64位浮点数 |
| `bool` | `boolean` | 布爾值 |
| `string` | `string` | 字串 |
| `array<T>` | `table` | 陣列 |
| `map<K, V>` | `table` | 對應 |
| `enum` | `number` | 列舉 |
| `struct` | `table` | 結構體 |

## 集成步骤

1. **安装 XLua**：在 Unity 專案中安装 XLua 外掛程式
2. **設定 XCell**：在專案設定中啟用 XLua 程式碼產生
3. **生成程式碼**：使用 XCell 生成 XLua 相容的程式碼
4. **載入資料**：在 Lua 指令碼中載入生成的資料

## 範例程式碼

```lua
-- 載入設定資料
local config = require("ConfigManager")

-- 存取設定
local playerConfig = config.Player[1]
print("Player name: " .. playerConfig.name)
print("Player level: " .. playerConfig.level)
```

## 注意事项

- XLua 中的数字類型统一為 `number`，可能會導致精度损失
- 復杂資料結構會被转换為 Lua table
- 建議使用 LuaJIT 以获得更好的效能