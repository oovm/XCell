/**
 * MonsterType数据结构
 */
export interface MonsterType {
    /**
     * id
     */
    id: number;
    /**
     * name
     */
    name: string;
    /**
     * description
     */
    description: string;
}

/**
 * MonsterType表加载器
 */
export class MonsterTypeTable {
    private items: MonsterType[] = [];

    /**
     * 加载MonsterType表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as MonsterType[];
        }
    }

    /**
     * 根据ID获取MonsterType
     * @param id MonsterTypeID
     */
    public getMonsterTypeById(id: number): MonsterType | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有MonsterType
     */
    public getAllMonsterType(): MonsterType[] {
        return this.items;
    }

}
