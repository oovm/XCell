/**
 * Characters数据结构
 */
export interface Characters {
    /**
     * id
     */
    id: number;
    /**
     * name
     */
    name: string;
    /**
     * age
     */
    age: number;
    /**
     * gender
     */
    gender: string;
    /**
     * personality
     */
    personality: string;
    /**
     * description
     */
    description: string;
    /**
     * image
     */
    image: string;
}

/**
 * Characters表加载器
 */
export class CharactersTable {
    private items: Characters[] = [];

    /**
     * 加载Characters表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as Characters[];
        }
    }

    /**
     * 根据ID获取Characters
     * @param id CharactersID
     */
    public getCharactersById(id: number): Characters | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Characters
     */
    public getAllCharacters(): Characters[] {
        return this.items;
    }
}
