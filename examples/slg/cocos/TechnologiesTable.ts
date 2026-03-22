/**
 * Technologies数据结构
 */
export interface Technologies {
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
    /**
     * research_time
     */
    research_time: number;
    /**
     * resource_cost
     */
    resource_cost: number;
    /**
     * required_building
     */
    required_building: number;
    /**
     * level_requirement
     */
    level_requirement: number;
    /**
     * effect
     */
    effect: string;
}

/**
 * Technologies表加载器
 */
export class TechnologiesTable {
    private items: Technologies[] = [];

    /**
     * 加载Technologies表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as Technologies[];
        }
    }

    /**
     * 根据ID获取Technologies
     * @param id TechnologiesID
     */
    public getTechnologiesById(id: number): Technologies | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Technologies
     */
    public getAllTechnologies(): Technologies[] {
        return this.items;
    }

}
