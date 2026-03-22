# SQL データベース統合

XCell は SQL ステートメントとデータの生成をサポートし、データベース統合に使用します。

## 型マッピング

| XCell 型 | SQL 型 | 説明 |
|-----------|----------|------|
| `i8` | `TINYINT` | 8ビット符号付き整数 |
| `i16` | `SMALLINT` | 16ビット符号付き整数 |
| `i32` | `INT` | 32ビット符号付き整数 |
| `i64` | `BIGINT` | 64ビット符号付き整数 |
| `u8` | `TINYINT UNSIGNED` | 8ビット符号なし整数 |
| `u16` | `SMALLINT UNSIGNED` | 16ビット符号なし整数 |
| `u32` | `INT UNSIGNED` | 32ビット符号なし整数 |
| `u64` | `BIGINT UNSIGNED` | 64ビット符号なし整数 |
| `f32` | `FLOAT` | 32ビット浮動小数点数 |
| `f64` | `DOUBLE` | 64ビット浮動小数点数 |
| `bool` | `BOOLEAN` | ブール値 |
| `string` | `VARCHAR(255)` | 文字列 |
| `array<T>` | `JSON` | 配列 |
| `map<K, V>` | `JSON` | マッピング |
| `enum` | `ENUM` | 列挙 |
| `struct` | `JSON` | 構造体 |

## サポートされるデータベース

- MySQL
- PostgreSQL
- SQLite
- Microsoft SQL Server
- Oracle

## 統合手順

1. **データベース接続の設定**：XCell 設定でデータベース接続情報を設定
2. **SQL の生成**：XCell を使用してテーブル作成ステートメントとデータ挿入ステートメントを生成
3. **SQL の実行**：データベース管理ツールで生成された SQL ステートメントを実行
4. **データの検証**：データが正しくデータベースにインポートされたことを確認

## サンプル SQL

```sql
-- テーブル作成
CREATE TABLE IF NOT EXISTS Player (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    level INT NOT NULL,
    gold BIGINT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE
);

-- データ挿入
INSERT INTO Player (id, name, level, gold, is_active) VALUES
(1, 'Player1', 10, 1000, TRUE),
(2, 'Player2', 15, 2000, TRUE),
(3, 'Player3', 20, 3000, FALSE);
```

## 注意事項

- 異なるデータベースシステムでは型に微妙な違いがある可能性があります
- 複雑なデータ構造には JSON 型の使用を推奨
- 生成された SQL ステートメントは特定のデータベースシステムに応じて調整が必要な場合があります
