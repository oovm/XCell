

/**
 * 怪物类型表数据结构
 */
export interface MonsterType {
    /**
     * 怪物类型变体
     */
    variant: string;
    /**
     * 怪物类型ID
     */
    id: string;
    /**
     * 怪物类型名称
     */
    name: string;
    /**
     * 怪物类型描述
     */
    description: string;
}

/**
 * 怪物类型表加载器
 */
export class MonsterTypeTable {
    private monsterTypes: MonsterType[] = [];

    /**
     * 加载怪物类型表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.monsterTypes = data;
        }
    }

    /**
     * 根据ID获取怪物类型
     * @param id 怪物类型ID
     */
    public getMonsterTypeById(id: string): MonsterType | null {
        return this.monsterTypes.find(type => type.id === id) || null;
    }

    /**
     * 根据变体获取怪物类型
     * @param variant 怪物类型变体
     */
    public getMonsterTypeByVariant(variant: string): MonsterType | null {
        return this.monsterTypes.find(type => type.variant === variant) || null;
    }

    /**
     * 获取所有怪物类型
     */
    public getAllMonsterTypes(): MonsterType[] {
        return this.monsterTypes;
    }
}
