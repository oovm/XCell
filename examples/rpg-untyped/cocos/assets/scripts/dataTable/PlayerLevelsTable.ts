namespace dataTable {
    /**
     * 玩家等级表数据结构
     */
    export interface PlayerLevel {
        /**
         * 等级ID
         */
        id: string;
        /**
         * 等级名称
         */
        name: string;
        /**
         * 所需经验值
         */
        required_exp: string;
        /**
         * 生命值
         */
        health: string;
        /**
         * 伤害值
         */
        damage: string;
        /**
         * 防御力
         */
        defense: string;
        /**
         * 解锁技能
         */
        unlock_skills: string;
    }

    /**
     * 玩家等级表加载器
     */
    export class PlayerLevelsTable {
        private static instance: PlayerLevelsTable;
        private playerLevels: PlayerLevel[] = [];

        /**
         * 获取单例实例
         */
        public static getInstance(): PlayerLevelsTable {
            if (!PlayerLevelsTable.instance) {
                PlayerLevelsTable.instance = new PlayerLevelsTable();
            }
            return PlayerLevelsTable.instance;
        }

        /**
         * 加载玩家等级表数据
         */
        public load(): void {
            const data = DataTableManager.getInstance().getTable<PlayerLevel[]>('PlayerLevels');
            if (data) {
                this.playerLevels = data;
            }
        }

        /**
         * 根据ID获取玩家等级
         * @param id 等级ID
         */
        public getPlayerLevelById(id: string): PlayerLevel | null {
            return this.playerLevels.find(level => level.id === id) || null;
        }

        /**
         * 获取所有玩家等级
         */
        public getAllPlayerLevels(): PlayerLevel[] {
            return this.playerLevels;
        }
    }
}
