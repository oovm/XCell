# SQL Database Integration

XCell supports generating SQL statements and data for database integration.

## Type Mapping

| XCell Type | SQL Type | Description |
| ---------- | -------- | ----------- |
| `i8` | `TINYINT` | 8-bit signed integer |
| `i16` | `SMALLINT` | 16-bit signed integer |
| `i32` | `INT` | 32-bit signed integer |
| `i64` | `BIGINT` | 64-bit signed integer |
| `u8` | `TINYINT UNSIGNED` | 8-bit unsigned integer |
| `u16` | `SMALLINT UNSIGNED` | 16-bit unsigned integer |
| `u32` | `INT UNSIGNED` | 32-bit unsigned integer |
| `u64` | `BIGINT UNSIGNED` | 64-bit unsigned integer |
| `f32` | `FLOAT` | 32-bit floating-point |
| `f64` | `DOUBLE` | 64-bit floating-point |
| `bool` | `BOOLEAN` | Boolean value |
| `string` | `VARCHAR(255)` | String |
| `array<T>` | `JSON` | Array |
| `map<K, V>` | `JSON` | Map |
| `enum` | `ENUM` | Enum |
| `struct` | `JSON` | Struct |

## Supported Databases

- MySQL
- PostgreSQL
- SQLite
- Microsoft SQL Server
- Oracle

## Integration Steps

1. **Configure Database Connection**: Set database connection information in XCell configuration
2. **Generate SQL**: Use XCell to generate table creation statements and data insert statements
3. **Execute SQL**: Execute generated SQL statements in database management tool
4. **Validate Data**: Confirm data has been correctly imported into database

## Example SQL

```sql
-- Create table
CREATE TABLE IF NOT EXISTS Player (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    level INT NOT NULL,
    gold BIGINT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE
);

-- Insert data
INSERT INTO Player (id, name, level, gold, is_active) VALUES
(1, 'Player1', 10, 1000, TRUE),
(2, 'Player2', 15, 2000, TRUE),
(3, 'Player3', 20, 3000, FALSE);
```

## Notes

- Different database systems may have slight type differences
- For complex data structures, it is recommended to use JSON type storage
- Generated SQL statements may need to be adjusted according to specific database systems
