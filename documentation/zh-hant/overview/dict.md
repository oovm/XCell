# dict 資料表

Dict 是最常用的設定形式，若没有特殊類型標記，預設视為 Dict 類型。Dict 以字串作為主鍵，适用於需要按名称存取的資料。

## 约定

- **第一列视為主鍵**
- **預設即為 Dict 類型，无需显式標記**
- **可使用 `@dict` 標記显式聲明**
- **生成的名称預設以檔案名為准，可通过 `@dict 名称` 指定**

## 標記方式

預設情況下无需標記，第一列自動识別為字串主鍵：

| 品质键 | 物品圖示 | 顯示顏色 |
|--------|----------|----------|
| quality_key | item_icon | display_color |
| string | string | string |
| common | item_01.png | #FFFFFF |
| rare | item_02.png | #00FFFF |
| epic | item_03.png | #FF00FF |

也可以显式使用 `@dict` 標記，放在第一行第一列：

| @dict | 物品圖示 | 顯示顏色 |
|--------|----------|----------|
| quality_key | item_icon | display_color |
| string | string | string |
| common | item_01.png | #FFFFFF |
| rare | item_02.png | #00FFFF |
| epic | item_03.png | #FF00FF |

## 程式碼產生

### TypeScript (Cocos, Laya)

```typescript
export class ItemQuality {
    public qualityKey: string;
    public itemIcon: string;
    public displayColor: string;
}

export class ItemQualityTable {
    private _items: Map<string, ItemQuality>;
}
```

### C# (Unity, Godot)

```csharp
public class ItemQuality
{
    public string QualityKey;
    public string ItemIcon;
    public string DisplayColor;
}

public class ItemQualityTable
{
    private Dictionary<string, ItemQuality> _items;
}
```

## 使用場景

品質設定、UI 設定、音效設定等需要按名称存取的資料。
