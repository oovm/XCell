

/**
 * 物品表数据结构
 */
export interface Item {
    /**
     * 物品ID
     */
    id: string;
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
    level: string;
    /**
     * 攻击力
     */
    attack: string;
    /**
     * 防御力
     */
    defense: string;
    /**
     * 物品描述
     */
    description: string;
}

/**
 * 物品表加载器
 */
export class ItemTable {
    private items: Item[] = [];

    /**
     * 加载物品表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data;
        }
    }

    /**
     * 根据ID获取物品
     * @param id 物品ID
     */
    public getItemById(id: string): Item | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有物品
     */
    public getAllItems(): Item[] {
        return this.items;
    }

    /**
     * 根据类型获取物品
     * @param type 物品类型
     */
    public getItemsByType(type: string): Item[] {
        return this.items.filter(item => item.type === type);
    }

    /**
     * 根据等级获取物品
     * @param level 物品等级
     */
    public getItemsByLevel(level: string): Item[] {
        return this.items.filter(item => item.level === level);
    }
}
