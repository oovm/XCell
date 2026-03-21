/**
 * 技能表数据结构
 */
export interface Skill {
    /**
     * 技能ID
     */
    id: number;
    /**
     * 技能名称
     */
    name: string;
    /**
     * 技能描述
     */
    description: string;
    /**
     * 技能伤害
     */
    damage: number;
    /**
     * 技能冷却时间
     */
    cooldown: number;
    /**
     * 技能消耗法力值
     */
    mana_cost: number;
    /**
     * 技能等级要求
     */
    level_requirement: number;
}

/**
 * 技能表加载器
 */
export class SkillsTable {
    private skills: Map<number, Skill> = new Map();

    /**
     * 加载技能表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            // 清空现有数据
            this.skills.clear();
            
            // 加载数据
            for (const skill of Object.values(data)) {
                const typedSkill = skill as Skill;
                this.skills.set(typedSkill.id, typedSkill);
            }
        }
    }

    /**
     * 根据ID获取技能
     * @param id 技能ID
     */
    public getSkillById(id: number): Skill | null {
        return this.skills.get(id) || null;
    }

    /**
     * 获取所有技能
     */
    public getAllSkills(): Skill[] {
        return Array.from(this.skills.values());
    }

    /**
     * 根据等级要求获取技能
     * @param level 玩家等级
     */
    public getSkillsByLevel(level: number): Skill[] {
        return Array.from(this.skills.values()).filter(skill => skill.level_requirement <= level);
    }
}