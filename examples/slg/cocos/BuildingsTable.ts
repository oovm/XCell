/**
 * Buildings数据结构
 */
export interface Buildings {
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
     * hp
     */
    hp: number;
    /**
     * build_time
     */
    build_time: number;
    /**
     * resource_cost
     */
    resource_cost: number;
    /**
     * effect
     */
    effect: string;
    /**
     * description
     */
    description: string;
}

/**
 * Buildings表加载器
 */
export class BuildingsTable {
    private items: Buildings[] = [];

    /**
     * 加载Buildings表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as Buildings[];
        }
    }

    /**
     * 根据ID获取Buildings
     * @param id BuildingsID
     */
    public getBuildingsById(id: number): Buildings | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Buildings
     */
    public getAllBuildings(): Buildings[] {
        return this.items;
    }

}
