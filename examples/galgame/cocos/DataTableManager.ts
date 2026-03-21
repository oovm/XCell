import { CharactersTable } from './CharactersTable';
import { DialoguesTable } from './DialoguesTable';
import { ScenesTable } from './ScenesTable';


/**
 * 数据表管理器
 * 负责加载和管理所有数据表
 */
export class DataTableManager {
    private static _instance: DataTableManager;

    // 惰性缓存字段
    private _charactersTable: CharactersTable | null = null;
    private _dialoguesTable: DialoguesTable | null = null;
    private _scenesTable: ScenesTable | null = null;
    
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
            this.getCharactersTable(),
            this.getDialoguesTable(),
            this.getScenesTable(),
        ]);
    }

    /**
     * 获取Characters表（惰性加载）
     */
    public async getCharactersTable(): Promise<CharactersTable> {
        if (this._charactersTable === null) {
            this._charactersTable = new CharactersTable();
            this._charactersTable.load(await this.loadJsonAsset('tables/Characters'));
        }
        return this._charactersTable;
    }

    /**
     * 获取Dialogues表（惰性加载）
     */
    public async getDialoguesTable(): Promise<DialoguesTable> {
        if (this._dialoguesTable === null) {
            this._dialoguesTable = new DialoguesTable();
            this._dialoguesTable.load(await this.loadJsonAsset('tables/Dialogues'));
        }
        return this._dialoguesTable;
    }

    /**
     * 获取Scenes表（惰性加载）
     */
    public async getScenesTable(): Promise<ScenesTable> {
        if (this._scenesTable === null) {
            this._scenesTable = new ScenesTable();
            this._scenesTable.load(await this.loadJsonAsset('tables/Scenes'));
        }
        return this._scenesTable;
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
