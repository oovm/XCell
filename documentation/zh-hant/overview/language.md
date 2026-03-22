# language 資料表

Language 資料表格用於管理多語言文本，XCell 提供了完整的阪际化支援。

## 约定

- **第一行第一列為 `@language` 標記**
- **可选 `@group` 欄位元用於分組**
- **其傭列為語言 ID**
- **生成的名称預設以檔案名為准，可通过 `@language 名称` 指定**

## 標記方式

第一行第一列使用 `@language` 標記，可选 `@group`，其傭為語言 ID：

| @language | @group | zh_cn | en_us | ja_jp |
|-----------|--------|-------|-------|-------|
| Ui_Start | ui | 開始 | Start | スタート |
| Ui_Settings | ui | 設定 | Settings | 設定 |
| Ui_Exit | ui | 結束 | Exit | 終了 |

## 程式碼生成

### TypeScript (Cocos, Laya)

```typescript
export class LanguageTable {
    private static _items: Map<string, string>;
    
    public static get(key: string): string {
        return this._items.get(key) ?? key;
    }
}
```

### C# (Unity, Godot)

```csharp
public static class LanguageTable
{
    private static Dictionary<string, string> _items;
    
    public static string Get(string key)
    {
        return _items.TryGetValue(key, out var value) ? value : key;
    }
}
```

## 使用場景

- 游戏 UI 多語言支援
- 套用程式阪际化
- 多語言文件系統
- 需要支援多种語言的任何專案
