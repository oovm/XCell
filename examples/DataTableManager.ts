import { ItemTable } from './ItemTable';
import { MonsterTable } from './MonsterTable';
import { PlayerLevelsTable } from './PlayerLevelsTable';
import { SkillTable } from './SkillTable';

/**
 * 数据表管理器
 * 负责加载和管理所有数据表
 */
export class DataTableManager {
    private static _instance: DataTableManager;

    // 惰性缓存字段
    private _itemTable: ItemTable | null = null;
    private _monsterTable: MonsterTable | null = null;
    private _playerLevelsTable: PlayerLevelsTable | null = null;
    private _skillTable: SkillTable | null = null;

    /**
     * 获取单例实例
     */
    public static getInstance(): DataTableManager {
        if (!DataTableManager._instance) {
            DataTableManager._instance = new DataTableManager();
        }
        return DataTableManager._instance;
    }

    /**
     * 加载所有数据表
     * 注意：表数据会在各自的表加载器中按需加载
     */
    public async loadAllTables(): Promise<void> {
        // 预加载所有表
        await Promise.all([
            this.getItemTable(),
            this.getMonsterTable(),
            this.getPlayerLevelsTable(),
            this.getSkillTable()
        ]);
    }

    /**
     * 获取物品表（惰性加载）
     */
    public async getItemTable(): Promise<ItemTable> {
        if (this._itemTable === null) {
            this._itemTable = new ItemTable();
            this._itemTable.load(await this.loadJsonAsset('tables/Item'));
        }
        return this._itemTable;
    }

    /**
     * 获取怪物表（惰性加载）
     */
    public async getMonsterTable(): Promise<MonsterTable> {
        if (this._monsterTable === null) {
            this._monsterTable = new MonsterTable();
            this._monsterTable.load(await this.loadJsonAsset('tables/Monster'));
        }
        return this._monsterTable;
    }

    /**
     * 获取玩家等级表（惰性加载）
     */
    public async getPlayerLevelsTable(): Promise<PlayerLevelsTable> {
        if (this._playerLevelsTable === null) {
            this._playerLevelsTable = new PlayerLevelsTable();
            this._playerLevelsTable.load(await this.loadJsonAsset('tables/PlayerLevels'));
        }
        return this._playerLevelsTable;
    }

    /**
     * 获取技能表（惰性加载）
     */
    public async getSkillTable(): Promise<SkillTable> {
        if (this._skillTable === null) {
            this._skillTable = new SkillTable();
            this._skillTable.load(await this.loadJsonAsset('tables/Skill'));
        }
        return this._skillTable;
    }

    /**
     * 加载JSON资源
     * @param path 资源路径
     */
    private async loadJsonAsset(path: string): Promise<cc.JsonAsset> {
        return new Promise<cc.JsonAsset>((resolve, reject) => {
            cc.resources.load(path, cc.JsonAsset, (err, asset) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(asset);
                }
            });
        });
    }
}
