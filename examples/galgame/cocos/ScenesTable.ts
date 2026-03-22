/**
 * Scenes数据结构
 */
export interface Scenes {
    /**
     * id
     */
    id: number;
    /**
     * name
     */
    name: string;
    /**
     * description
     */
    description: string;
    /**
     * background_image
     */
    background_image: string;
    /**
     * bgm
     */
    bgm: string;
}

/**
 * Scenes表加载器
 */
export class ScenesTable {
    private items: Scenes[] = [];

    /**
     * 加载Scenes表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as Scenes[];
        }
    }

    /**
     * 根据ID获取Scenes
     * @param id ScenesID
     */
    public getScenesById(id: number): Scenes | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Scenes
     */
    public getAllScenes(): Scenes[] {
        return this.items;
    }
}
