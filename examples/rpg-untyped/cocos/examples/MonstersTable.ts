

import { MonsterType } from "./MonsterType";

/**
 * 怪物表数据结构
 */
export interface Monster {
    /**
     * 怪物ID
     */
    id: number;
    /**
     * 怪物名称
     */
    name: string;
    /**
     * 怪物等级
     */
    level: number;
    /**
     * 怪物生命值
     */
    health: number;
    /**
     * 怪物伤害
     */
    damage: number;
    /**
     * 怪物防御力
     */
    defense: number;
    /**
     * 怪物类型
     */
    type: MonsterType;
    /**
     * 掉落物品
     */
    drop_items: number[];
    /**
     * 怪物技能
     */
    skills: number[];
}

/**
 * 怪物表加载器
 */
export class MonstersTable {
    private monsters: Map<number, Monster> = new Map();

    /**
     * 加载怪物表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            // 清空现有数据
            this.monsters.clear();
            
            // 加载数据
            for (const monster of Object.values(data)) {
                const typedMonster = monster as Monster;
                this.monsters.set(typedMonster.id, typedMonster);
            }
        }
    }

    /**
     * 根据ID获取怪物
     * @param id 怪物ID
     */
    public getMonsterById(id: number): Monster | null {
        return this.monsters.get(id) || null;
    }

    /**
     * 获取所有怪物
     */
    public getAllMonsters(): Monster[] {
        return Array.from(this.monsters.values());
    }

    /**
     * 根据类型获取怪物
     * @param type 怪物类型
     */
    public getMonstersByType(type: MonsterType): Monster[] {
        return Array.from(this.monsters.values()).filter(monster => monster.type === type);
    }

    /**
     * 根据等级获取怪物
     * @param level 怪物等级
     */
    public getMonstersByLevel(level: number): Monster[] {
        return Array.from(this.monsters.values()).filter(monster => monster.level === level);
    }
}
