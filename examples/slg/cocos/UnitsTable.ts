/**
 * Units数据结构
 */
export interface Units {
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
     * health
     */
    health: number;
    /**
     * attack
     */
    attack: number;
    /**
     * defense
     */
    defense: number;
    /**
     * movement
     */
    movement: number;
    /**
     * range
     */
    range: number;
    /**
     * cost
     */
    cost: number;
    /**
     * description
     */
    description: string;
}

/**
 * Units表加载器
 */
export class UnitsTable {
    private items: Units[] = [];

    /**
     * 加载Units表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as Units[];
        }
    }

    /**
     * 根据ID获取Units
     * @param id UnitsID
     */
    public getUnitsById(id: number): Units | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Units
     */
    public getAllUnits(): Units[] {
        return this.items;
    }

    /**
     * 根据类型获取Units
     * @param type 类型
     */
    public getUnitsByType(type: string): Units[] {
        return this.items.filter(item => item.type === type);
    }

    /**
     * 根据等级获取Units
     * @param level 等级
     */
    public getUnitsByLevel(level: number): Units[] {
        return this.items.filter(item => item.level === level);
    }

}
