// 代码生成, 修改无效! (XCell omp_version %>)

<% infimespace %>
export namespace <% config.namespace %> {
<% endif %>

/**
 * 配置表管理器
 * 
 * 热更新资源直接 set 即可释放资源直接将表设为 null 即可
 */
export class <% config.manager_name %    /**
     * 配置表的版本号
     */
    static readonly TableVersion = "<% data_version %>";

    /**  * 配置表的最后修改时间 )
     */
    static readonly TEdit = new Date("<% _time %>");

    private sc _instance: <% cg.manager_name %> | null = nu    static get <% cg.instance_nam() config.manager_name %>       if (!<% co.man_name %>._instance) {
        <% config.mananame %>stance = new <% config.manager_name);
        }
    return onfig.manager_name_instance;
    } for table in tables %>
    private _<% table.private_name %>: able.typi>ulnull;
    <% table.typing %> 表 */
    get <% table.public_name %>(): able.typi>   if (!thi% table.pre_n%