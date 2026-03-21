import { BuildingsTable } from './BuildingsTable';
import { TechnologiesTable } from './TechnologiesTable';
import { UnitsTable } from './UnitsTable';


/**
 * 数据表管理器
 * 负责加载和管理所有数据表
 */
export class DataTableManager {
    private static _instance: DataTableManager;

    // 惰性缓存字段
    private _buildingsTable: BuildingsTable | null = null;
    private _technologiesTable: TechnologiesTable | null = null;
    private _unitsTable: UnitsTable | null = null;
    
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
            this.getBuildingsTable(),
            this.getTechnologiesTable(),
            this.getUnitsTable(),
        ]);
    }

    /**
     * 获取Buildings表（惰性加载）
     */
    public async getBuildingsTable(): Promise<BuildingsTable> {
        if (this._buildingsTable === null) {
            this._buildingsTable = new BuildingsTable();
            this._buildingsTable.load(await this.loadJsonAsset('tables/Buildings'));
        }
        return this._buildingsTable;
    }

    /**
     * 获取Technologies表（惰性加载）
     */
    public async getTechnologiesTable(): Promise<TechnologiesTable> {
        if (this._technologiesTable === null) {
            this._technologiesTable = new TechnologiesTable();
            this._technologiesTable.load(await this.loadJsonAsset('tables/Technologies'));
        }
        return this._technologiesTable;
    }

    /**
     * 获取Units表（惰性加载）
     */
    public async getUnitsTable(): Promise<UnitsTable> {
        if (this._unitsTable === null) {
            this._unitsTable = new UnitsTable();
            this._unitsTable.load(await this.loadJsonAsset('tables/Units'));
        }
        return this._unitsTable;
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
