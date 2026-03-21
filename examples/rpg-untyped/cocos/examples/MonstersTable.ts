

/**
 * 怪物表数据结构
 */
export interface Monster {
    /**
     * 怪物ID
     */
    id: string;
    /**
     * 怪物名称
     */
    name: string;
    /**
     * 怪物等级
     */
    level: string;
    /**
     * 怪物生命值
     */
    health: string;
    /**
     * 怪物伤害
     */
    damage: string;
    /**
     * 怪物防御力
     */
    defense: string;
    /**
     * 怪物类型
     */
    type: string;
    /**
     * 掉落物品
     */
    drop_items: string;
    /**
     * 怪物技能
     */
    skills: string;
}

/**
 * 怪物表加载器
 */
export class MonstersTable {
    private monsters: Monster[] = [];

    /**
     * 加载怪物表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.monsters = data;
        }
    }

    /**
     * 根据ID获取怪物
     * @param id 怪物ID
     */
    public getMonsterById(id: string): Monster | null {
        return this.monsters.find(monster => monster.id === id) || null;
    }

    /**
     * 获取所有怪物
     */
    public getAllMonsters(): Monster[] {
        return this.monsters;
    }

    /**
     * 根据类型获取怪物
     * @param type 怪物类型
     */
    public getMonstersByType(type: string): Monster[] {
        return this.monsters.filter(monster => monster.type === type);
    }

    /**
     * 根据等级获取怪物
     * @param level 怪物等级
     */
    public getMonstersByLevel(level: string): Monster[] {
        return this.monsters.filter(monster => monster.level === level);
    }
}
