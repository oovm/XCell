/**
 * <% class_name %>数据结构
 */
export interface <% class_name %> {
{% for field in fields %}
    /**
     * <% field.name %>
     */
    <% field.name %>: <% field.type %>;
{% endfor %}
}

/**
 * <% class_name %>表加载器
 */
export class <% table_name %> {
    private items: <% class_name %>[] = [];

    /**
     * 加载<% class_name %>表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {
        const data = asset.json;
        if (data) {
            this.items = data as <% class_name %>[];
        }
    }

    /**
     * 根据ID获取<% class_name %>
     * @param id <% class_name %>ID
     */
    public get<% class_name %>ById(id: number): <% class_name %> | null {
        return this.items.find(item => item.id === id) || null;
    }

    /**
     * 获取所有<% class_name %>
     */
    public getAll<% class_name %>(): <% class_name %>[] {
        return this.items;
    }
{% if has_type_field %}
{% if is_monster %}
    /**
     * 根据类型获取<% class_name %>
     * @param type 怪物类型
     */
    public get<% class_name %>ByType(type: MonsterType): <% class_name %>[] {
        return this.items.filter(item => item.type === type);
    }
{% else %}
    /**
     * 根据类型获取<% class_name %>
     * @param type 类型
     */
    public get<% class_name %>ByType(type: string): <% class_name %>[] {
        return this.items.filter(item => item.type === type);
    }
{% endif %}
{% endif %}

}