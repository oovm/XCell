import { MonsterType } from "./MonsterType";

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
     */
    public async load(): Promise<void> {
        const path = 'tables/Monster';
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
     * 根据ID获取Monster
     * @param id MonsterID
     */
    public getMonsterById(id: string): Monster | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Monster
     */
    public getAllMonster(): Monster[] {
        return this.items;
    }

<% if has_type_field %>
    /**
     * 根据类型获取Monster
     * @param type 类型
     */
    public getMonsterByType(type: string): Monster[] {
        return this.items.filter(item => item.type === type);
    }
<% endif %>

<% if has_level_field %>
    /**
     * 根据等级获取Monster
     * @param level 等级
     */
    public getMonsterByLevel(level: string): Monster[] {
        return this.items.filter(item => item.level === level);
    }
<% endif %>
}