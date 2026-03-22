# 參照類型

## 為什麼需要參照類型？

在設定游戏資料时，经常會遇到这样的問題：

**ID 配错**

道具資料表里填農了品质 ID `comon`，但品质資料表里導际是 `common`，少了一個 `m`。这种拼農錯誤很难葉现，游戏執行时就會出問題。

**ID 改了，參照没同步**

品质資料表里的 `common` 改成了 `normal`，但道具資料表里还在用 `common`。结果就是道具找不到盡应的品质設定。

---

參照類型就是為了解凍这些問題而设计的。XCell 會自勁檢查參照是否正确，确保：
- 填農的 ID 一定存在於目标資料表中
- 如果 ID 不存在，會立即报错提示

## 基本格式

- `&T` - 指向 T 資料表的參照

## 什麼是參照？

简盧来说，參照就是"指向另一张資料表中某一行資料"的連結。

舉個例子：道具資料表中需要記錄"这個道具嵐於哪個品质"，这时就可以用參照類型指向品质資料表。

## 範例

### 第一步：建立品质資料表

先建立一张品质資料表：

| @dict | 顏色 | 描述 |
| ----- | ---- | ---- |
| quality_id | color | desc |
| string | string | string |
| common | #FFFFFF | 普通品质 |
| rare | #00FFFF | 稀有品质 |
| epic | #FF00FF | 史诗品质 |

### 第二步：建立道具資料表（參照品质資料表）

道具資料表中的 `quality_id` 欄位元使用 `&Quality` 類型，資料表示它參照的是品质資料表：

| @dict | 名称 | 品质 | 攻擊力 |
| ----- | ---- | ---- | ------ |
| item_id | name | quality_id | attack |
| string | string | &Quality | i32 |
| sword_001 | 铁劇 | common | 10 |
| sword_002 | 精钢劇 | rare | 50 |
| sword_003 | 龙牙劇 | epic | 200 |

## 常见使用場景

| 場景 | 说明 |
| ---- | ---- |
| 道具→品质 | 道具嵐於哪個品质等级 |
| 装獎→角色 | 装獎可以装獎在哪個角色上 |
| 技能→职業 | 技能嵐於哪個职業 |
| 怪物→掉落資料表 | 怪物掉落哪個掉落資料表 |


## 可參照的資料表格類型

以下類型的資料表格可以被參照：

| 類型 | 说明 |
| ---- | ---- |
| dict 資料表 | 字元串主鍵，如物品ID、技能ID |
| list 資料表 | 整数主鍵，如怪物编後、道具编後 |
| language 資料表 | 語言键，格式為 `組名/語言键`，如 `&Language` |

### language 資料表參照的特殊格式

參照 language 資料表时，填農的值格式為 `組名/語言键`：

| @dict | 名称 | 提示文字 |
| ----- | ---- | -------- |
| item_id | name | tip |
| string | string | &Language |
| sword_001 | 铁劇 | ui/item_tip_001 |

以下類型**不可參照**：

| 類型 | 原因 |
| ---- | ---- |
| enum 資料表 | 列舉是固定選項，不是資料資料表 |
| class 資料表 | 全局設定只有一個實例 |
| 基本類型 | i32、string 等是值類型，不是資料表 |


## 嵌套類型中的參照

參照類型可以與其他類型组合使用，形成更奧杂的資料结构。

### 參照陣列

当需要參照多個目标时，可以使用參照陣列：

| 格式 | 说明 |
| ---- | ---- |
| `[&T]` | 參照陣列，儲存多個目标資料表的主鍵 |

**範例：掉落資料表參照多個道具**

| @dict | 名称 | 掉落道具 |
| ----- | ---- | -------- |
| drop_id | name | items |
| string | string | [&Item] |
| drop_001 | 新手礼套件 | sword_001, potion_001 |
| drop_002 | 精英婁勞 | sword_002, armor_001, gem_001 |

### 其他嵌套组合

| 格式 | 说明 |
| ---- | ---- |
| `[&T]` | 參照陣列 |
| `Vec<&T>` | 參照向量 |
| `HashMap<string, &T>` | 字元串到參照的對應 |

## 參照驗證

XCell 會自勁檢查參照是否正确：

- ✅ 參照的ID必须存在於目标資料表中
- ✅ 不能填農不存在的ID
- ✅ 迴圈參照會被检测并提示

## 程式碼生成

### TypeScript (Cocos, Laya)

```typescript
export class Item {
    public itemId: string;
    public name: string;
    public qualityId: string;  // 儲存主鍵值
    public attack: number;
}

export class ItemTable {
    private _items: Map<string, Item>;
    private _qualityTable: QualityTable;
    
    public getQuality(itemId: string): Quality | undefined {
        const item = this._items.get(itemId);
        if (item) {
            return this._qualityTable.get(item.qualityId);
        }
        return undefined;
    }
}
```

### C# (Unity, Godot)

```csharp
public class Item
{
    public string ItemId;
    public string Name;
    public string QualityId;  // 儲存主鍵值
    public int Attack;
}

public class ItemTable
{
    private Dictionary<string, Item> _items;
    private QualityTable _qualityTable;
    
    public Quality GetQuality(string itemId)
    {
        if (_items.TryGetValue(itemId, out var item))
        {
            return _qualityTable.Get(item.QualityId);
        }
        return null;
    }
}
```

## 注意事项

- 參照的目标資料表必须存在
- 目标資料表必须是 dict、list 或 language 類型
- enum、class 資料表不能被參照
- 基本類型（i32、string 等）不能被參照
- 資料表名建議使用大驼峰命名，如 `&QualityTable`
