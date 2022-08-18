// 代码生成, 修改无效!
// ReSharper disable ArrangeObjectCreationWhenTypeEvident
// ReSharper disable RedundantAbstractModifier
// ReSharper disable PartialTypeWithSinglePart
// ReSharper disable CheckNamespace

using System;
using System.IO;

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
        public const string TableVersion = "{{ config.table_version }}";

        /// <summary>
        /// 配置表的最后修改时间
        /// </summary>
        /// <remarks>自动生成</remarks>
        public DateTime TableEditTime = {{ edit_time }};

        private static readonly Lazy<{{ config.manager_name }}> singleton = new(() => new {{ config.manager_name }}());
        public static {{ config.manager_name }} {{ config.instance_name }} => singleton.Value;
{% for table in tables %}
        private {{ table.type }} {{ table.private }};
        /// <inheritdoc cref="{{config.namespace}}.{{ table.type }}"/>
        public {{ table.type }} {{ table.public }}
        {
{%- if config.legacy_null_null %}
            get { return {{ table.private }} != null ? {{ table.private }} : new {{ table.type }}(); }
{%- else %}
            get => {{ table.private }} ?? new {{ table.type }}();
{%- endif %}
            set => {{ table.private }} = value;
        }
{% endfor %}

        public void LoadAll()
        {
{%- for table in tables %}
			{{ table.private }} = new {{ table.type }}();
{%- endfor %}
        }

        public void Clear()
        {
{%- for table in tables %}
			{{ table.private }} = null;
{%- endfor %}
        }
    }

    public interface IBinarySupport
    {
        /// <summary>
        /// 以小端序 (LittleEndian) 读取二进制数据
        /// </summary>
        /// <param name="r">二进制读取流</param>
        public abstract void BinaryRead(BinaryReader r);

        /// <summary>
        /// 以小端序 (LittleEndian) 写入二进制数据
        /// </summary>
        /// <param name="w">二进制写入流</param>
        public abstract void BinaryWrite(BinaryWriter w);
    }

    public interface IXmlSupport
    {
        /// <summary>
        /// 读取 XML
        /// </summary>
        /// <param name="r"></param>
        public abstract void XmlRead(BinaryReader r);
        /// <summary>
        /// 写入 XML
        /// </summary>
        /// <param name="w"></param>
        public abstract void XmlWrite(BinaryWriter w);
    }
}