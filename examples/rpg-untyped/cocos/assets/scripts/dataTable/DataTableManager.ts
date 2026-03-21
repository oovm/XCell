namespace dataTable {
    /**
     * 数据表管理器
     * 负责加载和管理所有数据表
     */
    export class DataTableManager {
        private static instance: DataTableManager;
        private tables: Map<string, any> = new Map();

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
         */
        public async loadAllTables(): Promise<void> {
            await this.loadTable('Item');
            await this.loadTable('MonsterType');
            await this.loadTable('Monsters');
            await this.loadTable('Placeholder');
            await this.loadTable('PlayerLevels');
            await this.loadTable('Skills');
        }

        /**
         * 加载指定数据表
         * @param tableName 表名
         */
        private async loadTable(tableName: string): Promise<void> {
            const path = `tables/${tableName}`;
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
            this.tables.set(tableName, data);
        }

        /**
         * 获取指定数据表
         * @param tableName 表名
         */
        public getTable<T>(tableName: string): T {
            return this.tables.get(tableName) as T;
        }
    }
}
