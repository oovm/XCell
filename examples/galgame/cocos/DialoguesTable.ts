/**
 * Dialogues数据结构
 */
export interface Dialogues {
    /**
     * id
     */
    id: number;
    /**
     * scene_id
     */
    scene_id: number;
    /**
     * character_id
     */
    character_id: number;
    /**
     * text
     */
    text: string;
    /**
     * choice_1
     */
    choice_1: string;
    /**
     * choice_2
     */
    choice_2: string;
    /**
     * choice_3
     */
    choice_3: string;
    /**
     * next_dialogue_id
     */
    next_dialogue_id: number;
}

/**
 * Dialogues表加载器
 */
export class DialoguesTable {
    private items: Dialogues[] = [];

    /**
     * 加载Dialogues表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as Dialogues[];
        }
    }

    /**
     * 根据ID获取Dialogues
     * @param id DialoguesID
     */
    public getDialoguesById(id: number): Dialogues | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有Dialogues
     */
    public getAllDialogues(): Dialogues[] {
        return this.items;
    }

}
