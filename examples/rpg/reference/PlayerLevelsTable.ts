

/**
 * 玩家等级表数据结构
 */
export interface PlayerLevel {
    /**
     * 等级ID
     */
    id: number;
    /**
     * 等级名称
     */
    name: string;
    /**
     * 所需经验值
     */
    required_exp: number;
    /**
     * 生命值
     */
    health: number;
    /**
     * 伤害值
     */
    damage: number;
    /**
     * 防御力
     */
    defense: number;
    /**
     * 解锁技能
     */
    unlock_skills: number[];
}

/**
 * 玩家等级表加载器
 */
export class PlayerLevelsTable {
    private playerLevels: Map<number, PlayerLevel> = new Map();

    /**
     * 加载玩家等级表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            // 清空现有数据
            this.playerLevels.clear();
            
            // 加载数据
            for (const level of Object.values(data)) {
                const typedLevel = level as PlayerLevel;
                this.playerLevels.set(typedLevel.id, typedLevel);
            }
        }
    }

    /**
     * 根据ID获取玩家等级
     * @param id 等级ID
     */
    public getPlayerLevelById(id: number): PlayerLevel | null {
        return this.playerLevels.get(id) || null;
    }

    /**
     * 获取所有玩家等级
     */
    public getAllPlayerLevels(): PlayerLevel[] {
        return Array.from(this.playerLevels.values());
    }
}
