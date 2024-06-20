# TypeScript/JavaScript 集成

XCell 支持生成 TypeScript 和 JavaScript 代码，用于前端和后端应用。

## 类型映射

| XCell 类型 | TypeScript 类型 | 说明 |
|-----------|----------------|------|
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
| `array<T>` | `T[]` | 数组 |
| `map<K, V>` | `Record<K, V>` | 映射 |
| `enum` | `string` | 枚举（字符串形式） |
| `struct` | `interface` | 结构体 |

## 支持的格式

- **TypeScript + JSON**：生成 TypeScript 接口和 JSON 数据文件
- **TypeScript + CSV**：生成 TypeScript 接口和 CSV 数据文件
- **JavaScript + JSON**：生成 JavaScript 代码和 JSON 数据文件
- **JavaScript + CSV**：生成 JavaScript 代码和 CSV 数据文件

## 集成步骤

1. **配置 TypeScript 导出**：在 XCell 配置中启用 TypeScript 格式导出
2. **生成代码**：使用 XCell 生成 TypeScript/JavaScript 代码和数据文件
3. **引入代码**：在项目中引入生成的代码和数据
4. **使用数据**：在应用程序中使用生成的数据和类型

## 示例 TypeScript 代码

```typescript
// 生成的类型定义
export interface Player {
  id: number;
  name: string;
  level: number;
  gold: number;
  is_active: boolean;
}

export interface Item {
  id: number;
  name: string;
  damage: number;
  price: number;
}

export interface ConfigData {
  Player: Player[];
  Item: Item[];
}

// 加载数据
import configData from './config.json';

// 使用数据
const player = configData.Player[0];
console.log(`Player name: ${player.name}`);
console.log(`Player level: ${player.level}`);
```

## 注意事项

- TypeScript 中的数字类型统一为 `number`，可能会导致精度损失
- 生成的接口可以直接用于类型检查和代码提示
- 可以配置生成不同风格的代码（ES modules、CommonJS 等）
- 对于大型项目，建议使用代码分割来优化加载性能