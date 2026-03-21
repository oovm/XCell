import { ItemTable } from "./ItemTable";
import { MonsterTypeTable } from "./MonsterTypeTable";
import { MonstersTable } from "./MonstersTable";
import { PlayerLevelsTable } from "./PlayerLevelsTable";
import { SkillsTable } from "./SkillsTable";

/**
 * 数据表管理器
 * 负责加载和管理所有数据表
 */
export class DataTableManager {
    private static instance: DataTableManager;

    // 惰性缓存字段
    private _itemTable: ItemTable | undefined = undefined;
    private _monsterTypeTable: MonsterTypeTable | undefined = undefined;
    private _monstersTable: MonstersTable | undefined = undefined;
    private _playerLevelsTable: PlayerLevelsTable | undefined = undefined;
    private _skillsTable: SkillsTable | undefined = undefined;

    /**
     * 获取单例实例
     */
    public static getInstance(): DataTableManager {
        if (!DataTableManager.instance) {
            DataTableManager.instance = new DataTableManager();
        }
        return DataTableManager.instance;
    }

    /**
     * 加载所有数据表
     * 注意：表数据会在各自的表加载器中按需加载
     */
    public async loadAllTables(): Promise<void> {
        // 预加载所有表
        await Promise.all([
            this.getItemTable(),
            this.getMonsterTypeTable(),
            this.getMonstersTable(),
            this.getPlayerLevelsTable(),
            this.getSkillsTable()
        ]);
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

    /**
     * 获取物品表（惰性加载）
     */
    public async getItemTable(): Promise<ItemTable> {
        if (!this._itemTable) {
            const asset = await this.loadJsonAsset('tables/Item');
            this._itemTable = new ItemTable();
            this._itemTable.load(asset);
        }
        return this._itemTable;
    }

    /**
     * 获取怪物类型表（惰性加载）
     */
    public async getMonsterTypeTable(): Promise<MonsterTypeTable> {
        if (!this._monsterTypeTable) {
            const asset = await this.loadJsonAsset('tables/MonsterType');
            this._monsterTypeTable = new MonsterTypeTable();
            this._monsterTypeTable.load(asset);
        }
        return this._monsterTypeTable;
    }

    /**
     * 获取怪物表（惰性加载）
     */
    public async getMonstersTable(): Promise<MonstersTable> {
        if (!this._monstersTable) {
            const asset = await this.loadJsonAsset('tables/Monsters');
            this._monstersTable = new MonstersTable();
            this._monstersTable.load(asset);
        }
        return this._monstersTable;
    }

    /**
     * 获取玩家等级表（惰性加载）
     */
    public async getPlayerLevelsTable(): Promise<PlayerLevelsTable> {
        if (!this._playerLevelsTable) {
            const asset = await this.loadJsonAsset('tables/PlayerLevels');
            this._playerLevelsTable = new PlayerLevelsTable();
            this._playerLevelsTable.load(asset);
        }
        return this._playerLevelsTable;
    }

    /**
     * 获取技能表（惰性加载）
     */
    public async getSkillsTable(): Promise<SkillsTable> {
        if (!this._skillsTable) {
            const asset = await this.loadJsonAsset('tables/Skills');
            this._skillsTable = new SkillsTable();
            this._skillsTable.load(asset);
        }
        return this._skillsTable;
    }
}
