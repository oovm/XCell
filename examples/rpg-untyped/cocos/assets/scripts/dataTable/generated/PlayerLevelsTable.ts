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
     */
    public async load(): Promise<void> {
        const path = 'tables/PlayerLevels';
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
     * 根据ID获取PlayerLevels
     * @param id PlayerLevelsID
     */
    public getPlayerLevelsById(id: string): PlayerLevels | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有PlayerLevels
     */
    public getAllPlayerLevels(): PlayerLevels[] {
        return this.items;
    }

<% if has_type_field %>
    /**
     * 根据类型获取PlayerLevels
     * @param type 类型
     */
    public getPlayerLevelsByType(type: string): PlayerLevels[] {
        return this.items.filter(item => item.type === type);
    }
<% endif %>

<% if has_level_field %>
    /**
     * 根据等级获取PlayerLevels
     * @param level 等级
     */
    public getPlayerLevelsByLevel(level: string): PlayerLevels[] {
        return this.items.filter(item => item.level === level);
    }
<% endif %>
}