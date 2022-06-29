// 代码生成, 修改无效!
// 当前版本: v{{ VERSION }}
// 查看更新: https://github.com/oovm/XCell
// ReSharper disable ArrangeObjectCreationWhenTypeEvident

using System;
using DataTable.Generated;

namespace {{ config.namespace }}
{
    /// <summary>
    /// 热更新资源直接 set 即可
    ///
    /// 释放资源直接将表设为 null 即可
    /// </summary>
    public partial class {{ config.manager_name }}
    {
        /// <summary>
        /// 配置表的版本号
        /// </summary>
        /// <remarks>手动配置</remarks>
        public const string TableVersion = "0.0.1";

        /// <summary>
        /// 配置表的最后修改时间
        /// </summary>
        /// <remarks>自动生成</remarks>
        public DateTime TableEditTime = new DateTime(2020, 1, 1, 0, 0, 0);

        private static readonly Lazy<{{ config.manager_name }}> singleton = new(() => new {{ config.manager_name }}());
        public static {{ config.manager_name }} Instance => singleton.Value;

{% for field in config.tables %}
        private {{ table.name }} {{ table.private }};
        /// <inheritdoc cref="{{config.namespace}}.{{ table.name }}"/>
        public {{ table.name }} {{ table.name }}
        {
{%- if config.legacy_null_null %}
            get { return {{ table.private }} != null ? {{ table.private }} : new {{ table.name }}(); }
{% else %}
            get => {{ table.private }} ?? new {{ table.name }}();
{%- endif %}
            set => {{ table.private }} = value;
        }
{% endfor %}

        public void LoadAll()
        {
{%- for field in CLASS_FIELDS %}
			{{ table.private }} = new {{ table.name }}();
{%- endfor %}
        }
    }
}