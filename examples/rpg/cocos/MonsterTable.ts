import { MonsterType } from './MonsterType';

/**
 * Monster数据结构
 */
export interface Monster {
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
    type: MonsterType;
    /**
     * level
     */
    level: number;
    /**
     * hp
     */
    hp: number;
    /**
     * attack
     */
    attack: number;
    /**
     * defense
     */
    defense: number;
    /**
     * drop_items
     */
    drop_items: number[];
    /**
     * skills
     */
    skills: number[];
}

/**
 * Monster表加载器
 */
export class MonsterTable {
    private items: Monster[] = [];

    /**
     * 加载Monster表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as Monster[];
        }
    }

    /**
     * 根据ID获取Monster
     * @param id MonsterID
     */
    public getMonsterById(id: number): Monster | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Monster
     */
    public getAllMonster(): Monster[] {
        return this.items;
    }

    /**
     * 根据类型获取Monster
     * @param type 怪物类型
     */
    public getMonsterByType(type: MonsterType): Monster[] {
        return this.items.filter(item => item.type === type);
    }

    /**
     * 根据等级获取Monster
     * @param level 等级
     */
    public getMonsterByLevel(level: number): Monster[] {
        return this.items.filter(item => item.level === level);
    }

}
