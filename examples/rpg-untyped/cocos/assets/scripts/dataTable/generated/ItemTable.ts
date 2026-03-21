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
     */
    public async load(): Promise<void> {
        const path = 'tables/Item';
        const asset = await new Promise<cc.JsonAsset>((resolve, reject) => {
            cc.resources.load(path, cc.JsonAsset, (err, asset) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(asset);
                }
            });
        });

        const data = asset.json;
        if (data) {
            this.items = data;
        }
    }

    /**
     * 根据ID获取Item
     * @param id ItemID
     */
    public getItemById(id: string): Item | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Item
     */
    public getAllItem(): Item[] {
        return this.items;
    }

<% if has_type_field %>
    /**
     * 根据类型获取Item
     * @param type 类型
     */
    public getItemByType(type: string): Item[] {
        return this.items.filter(item => item.type === type);
    }
<% endif %>

<% if has_level_field %>
    /**
     * 根据等级获取Item
     * @param level 等级
     */
    public getItemByLevel(level: string): Item[] {
        return this.items.filter(item => item.level === level);
    }
<% endif %>
}