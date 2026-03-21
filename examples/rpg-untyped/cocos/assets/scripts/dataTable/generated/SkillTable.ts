/**
 * Skill数据结构
 */
export interface Skill {
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
     * level_requirement
     */
    level_requirement: number;
    /**
     * mp_cost
     */
    mp_cost: number;
    /**
     * damage
     */
    damage: number;
    /**
     * description
     */
    description: string;

}

/**
 * Skill表加载器
 */
export class SkillTable {
    private items: Skill[] = [];

    /**
     * 加载Skill表数据
     */
    public async load(): Promise<void> {
        const path = 'tables/Skill';
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
     * 根据ID获取Skill
     * @param id SkillID
     */
    public getSkillById(id: string): Skill | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Skill
     */
    public getAllSkill(): Skill[] {
        return this.items;
    }

<% if has_type_field %>
    /**
     * 根据类型获取Skill
     * @param type 类型
     */
    public getSkillByType(type: string): Skill[] {
        return this.items.filter(item => item.type === type);
    }
<% endif %>

<% if has_level_field %>
    /**
     * 根据等级获取Skill
     * @param level 等级
     */
    public getSkillByLevel(level: string): Skill[] {
        return this.items.filter(item => item.level === level);
    }
<% endif %>
}