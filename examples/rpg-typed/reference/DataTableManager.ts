import { ItemTable } from "./ItemTable";
import { MonstersTable } from "./MonstersTable";
import { PlayerLevelsTable } from "./PlayerLevelsTable";
import { SkillsTable } from "./SkillsTable";

/**
 * 数据表管理器
 * 负责加载和管理所有数据表
 */
export class DataTableManager {
    private static _instance: DataTableManager;

    // 惰性缓存字段
    private _itemTable: ItemTable | null = null;
    private _monstersTable: MonstersTable | null = null;
    private _playerLevelsTable: PlayerLevelsTable | null = null;
    private _skillsTable: SkillsTable | null = null;

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
            this.getMonstersTable(),
            this.getPlayerLevelsTable(),
            this.getSkillsTable()
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
    public async getMonstersTable(): Promise<MonstersTable> {
        if (this._monstersTable === null) {
            this._monstersTable = new MonstersTable();
            this._monstersTable.load(await this.loadJsonAsset('tables/Monsters'));
        }
        return this._monstersTable;
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
    public async getSkillsTable(): Promise<SkillsTable> {
        if (this._skillsTable === null) {
            this._skillsTable = new SkillsTable();
            this._skillsTable.load(await this.loadJsonAsset('tables/Skills'));
        }
        return this._skillsTable;
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
