{% for table in tables %}
import { {{ table.table_name }} } from './{{ table.table_name }}';
{% endfor %}


/**
 * 数据表管理器
 * 负责加载和管理所有数据表
 */
export class DataTableManager {
    private static _instance: DataTableManager;

    // 惰性缓存字段
{% for table in tables %}
    private _{{ table.cache_name }}: {{ table.table_name }} | null = null;
{% endfor %}
    
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
{% for table in tables %}
            this.{{ table.get_method_name }}(),
{% endfor %}
        ]);
    }
{% for table in tables %}

    /**
     * 获取{{ table.class_name }}表（惰性加载）
     */
    public async {{ table.get_method_name }}(): Promise<{{ table.table_name }}> {
        if (this._{{ table.cache_name }} === null) {
            this._{{ table.cache_name }} = new {{ table.table_name }}();
            this._{{ table.cache_name }}.load(await this.loadJsonAsset('{{ table_data_path }}{{ table.class_name }}'));
        }
        return this._{{ table.cache_name }};
    }
{% endfor %}


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