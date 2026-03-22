# JSON Data Format

JSON is a lightweight data interchange format. XCell supports exporting configuration data to JSON format.

## Type Mapping

| XCell Type | JSON Type | Description |
| ---------- | --------- | ----------- |
| `i8` | `number` | 8-bit signed integer |
| `i16` | `number` | 16-bit signed integer |
| `i32` | `number` | 32-bit signed integer |
| `i64` | `number` | 64-bit signed integer |
| `u8` | `number` | 8-bit unsigned integer |
| `u16` | `number` | 16-bit unsigned integer |
| `u32` | `number` | 32-bit unsigned integer |
| `u64` | `number` | 64-bit unsigned integer |
| `f32` | `number` | 32-bit floating-point |
| `f64` | `number` | 64-bit floating-point |
| `bool` | `boolean` | Boolean value |
| `string` | `string` | String |
| `array<T>` | `array` | Array |
| `map<K, V>` | `object` | Map |
| `enum` | `string` | Enum (string form) |
| `struct` | `object` | Struct |

## Integration Steps

1. **Configure JSON Export**: Enable JSON format export in XCell configuration
2. **Generate JSON**: Use XCell to generate JSON data files
3. **Load JSON**: Load and parse JSON files in target platform
4. **Use Data**: Use parsed data in application

## Example JSON

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

## Notes

- JSON number types have no precision limit, but may have precision issues in some languages
- Complex data structures will be correctly converted to nested JSON objects and arrays
- Enum values are stored as strings by default, but can also be configured as numeric form
- Generated JSON files can be used directly in frontend and backend applications
