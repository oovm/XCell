/**
 * PlayerLevels数据结构
 */
export interface PlayerLevels {
    /**
     * id
     */
    id: number;
    /**
     * level
     */
    level: number;
    /**
     * exp_required
     */
    exp_required: number;
    /**
     * hp
     */
    hp: number;
    /**
     * mp
     */
    mp: number;
    /**
     * attack
     */
    attack: number;
    /**
     * defense
     */
    defense: number;
    /**
     * unlock_skills
     */
    unlock_skills: number[];
}

/**
 * PlayerLevels表加载器
 */
export class PlayerLevelsTable {
    private items: PlayerLevels[] = [];

    /**
     * 加载PlayerLevels表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as PlayerLevels[];
        }
    }

    /**
     * 根据ID获取PlayerLevels
     * @param id PlayerLevelsID
     */
    public getPlayerLevelsById(id: number): PlayerLevels | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有PlayerLevels
     */
    public getAllPlayerLevels(): PlayerLevels[] {
        return this.items;
    }

    /**
     * 根据等级获取PlayerLevels
     * @param level 等级
     */
    public getPlayerLevelsByLevel(level: number): PlayerLevels[] {
        return this.items.filter(item => item.level === level);
    }

}
