

/**
 * 物品表数据结构
 */
export interface Item {
    /**
     * 物品ID
     */
    id: number;
    /**
     * 物品名称
     */
    name: string;
    /**
     * 物品类型
     */
    type: string;
    /**
     * 物品等级
     */
    level: number;
    /**
     * 攻击力
     */
    attack: number;
    /**
     * 防御力
     */
    defense: number;
    /**
     * 物品描述
     */
    description: string;
}

/**
 * 物品表加载器
 */
export class ItemTable {
    private items: Map<number, Item> = new Map();

    /**
     * 加载物品表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            // 清空现有数据
            this.items.clear();
            
            // 加载数据
            for (const item of Object.values(data)) {
                const typedItem = item as Item;
                this.items.set(typedItem.id, typedItem);
            }
        }
    }

    /**
     * 根据ID获取物品
     * @param id 物品ID
     */
    public getItemById(id: number): Item | null {
        return this.items.get(id) || null;
    }

    /**
     * 获取所有物品
     */
    public getAllItems(): Item[] {
        return Array.from(this.items.values());
    }

    /**
     * 根据类型获取物品
     * @param type 物品类型
     */
    public getItemsByType(type: string): Item[] {
        return Array.from(this.items.values()).filter(item => item.type === type);
    }

    /**
     * 根据等级获取物品
     * @param level 物品等级
     */
    public getItemsByLevel(level: number): Item[] {
        return Array.from(this.items.values()).filter(item => item.level === level);
    }
}
