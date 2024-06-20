# SQL 数据库集成

XCell 支持生成 SQL 语句和数据，用于数据库集成。

## 类型映射

| XCell 类型 | SQL 类型 | 说明 |
|-----------|----------|------|
| `i8` | `TINYINT` | 8位有符号整数 |
| `i16` | `SMALLINT` | 16位有符号整数 |
| `i32` | `INT` | 32位有符号整数 |
| `i64` | `BIGINT` | 64位有符号整数 |
| `u8` | `TINYINT UNSIGNED` | 8位无符号整数 |
| `u16` | `SMALLINT UNSIGNED` | 16位无符号整数 |
| `u32` | `INT UNSIGNED` | 32位无符号整数 |
| `u64` | `BIGINT UNSIGNED` | 64位无符号整数 |
| `f32` | `FLOAT` | 32位浮点数 |
| `f64` | `DOUBLE` | 64位浮点数 |
| `bool` | `BOOLEAN` | 布尔值 |
| `string` | `VARCHAR(255)` | 字符串 |
| `array<T>` | `JSON` | 数组 |
| `map<K, V>` | `JSON` | 映射 |
| `enum` | `ENUM` | 枚举 |
| `struct` | `JSON` | 结构体 |

## 支持的数据库

- MySQL
- PostgreSQL
- SQLite
- Microsoft SQL Server
- Oracle

## 集成步骤

1. **配置数据库连接**：在 XCell 配置中设置数据库连接信息
2. **生成 SQL**：使用 XCell 生成建表语句和数据插入语句
3. **执行 SQL**：在数据库管理工具中执行生成的 SQL 语句
4. **验证数据**：确认数据已正确导入数据库

## 示例 SQL

```sql
-- 创建表
CREATE TABLE IF NOT EXISTS Player (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    level INT NOT NULL,
    gold BIGINT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE
);

-- 插入数据
INSERT INTO Player (id, name, level, gold, is_active) VALUES
(1, 'Player1', 10, 1000, TRUE),
(2, 'Player2', 15, 2000, TRUE),
(3, 'Player3', 20, 3000, FALSE);
```

## 注意事项

- 不同数据库系统可能有细微的类型差异
- 对于复杂数据结构，建议使用 JSON 类型存储
- 生成的 SQL 语句可能需要根据具体数据库系统进行调整