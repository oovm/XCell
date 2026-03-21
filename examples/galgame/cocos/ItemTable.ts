/**
 * Item数据结构
 */
export interface Item {
    /**
     * id
     */
    id: number;
    /**
     * name
     */
    name: string;
    /**
     * type
     */
    type: string;
    /**
     * level
     */
    level: number;
    /**
     * attack
     */
    attack: number;
    /**
     * defense
     */
    defense: number;
    /**
     * description
     */
    description: string;
}

/**
 * Item表加载器
 */
export class ItemTable {
    private items: Item[] = [];

    /**
     * 加载Item表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as Item[];
        }
    }

    /**
     * 根据ID获取Item
     * @param id ItemID
     */
    public getItemById(id: number): Item | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Item
     */
    public getAllItem(): Item[] {
        return this.items;
    }

    /**
     * 根据类型获取Item
     * @param type 类型
     */
    public getItemByType(type: string): Item[] {
        return this.items.filter(item => item.type === type);
    }

    /**
     * 根据等级获取Item
     * @param level 等级
     */
    public getItemByLevel(level: number): Item[] {
        return this.items.filter(item => item.level === level);
    }

}
