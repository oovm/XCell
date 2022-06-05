// 代码生成, 修改无效!
// 当前版本: v{{ VERSION }}
// 查看更新: https://github.com/oovm/XCell
// ReSharper disable RedundantDefaultMemberInitializer
// ReSharper disable RedundantUsingDirective

using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization;
using JetBrains.Annotations;

namespace {{ NAMESPACE }}
{
    [DataContract]
    public partial class {{ TABLE_NAME }}
    {
        [DataMember] public readonly Dictionary<int, {{ELEMENT_NAME}}> dict = new();

        [CanBeNull]
        public {{ELEMENT_NAME}} {{ELEMENT_GETTER}}(int id)
        {
            return dict.TryGetValue(id, out var item) ? item : null;
        }
    }

    [DataContract]
    public partial class {{ELEMENT_NAME}}
    {
{%- for field in CLASS_FIELDS %}
        /// <summary>
{%- for line in field.summary %}
        /// {{line}}
{%- endfor %}
        /// </summary>
        /// <remarks>
{%- for line in field.remarks %}
        /// {{line}}
{%- endfor %}
        /// </remarks>
        [DataMember]
{%- if field.has_default %}
        public {{field.typing}} {{field.name}} = {{field.default}};
{% else %}
        public {{field.typing}} {{field.name}};
{%- endif %}
{%- endfor %}
    }
{% if SUPPORT_BINARY %}
    public partial class {{TABLE_NAME}} : IBinarySupport
    {
        /// <summary>
        /// 从二进制文件中读取静态数据
        /// </summary>
        /// <param name="path">二进制文件包路径</param>
        ///
        /// <example>
        /// <code>
        /// AssetDatabase.GetAssetPath    // 根据 Asset 路径
        /// AssetDatabase.GUIDToAssetPath // 根据 GUID
        /// </code>
        /// </example>
        public void BinaryRead(string path)
        {
            using var stream = File.Open(path, FileMode.Open);
            using var reader = new BinaryReader(stream, Encoding.UTF8, false);
            BinaryRead(reader);
        }

		/// <inheritdoc cref="IBinarySupport.BinaryRead"/>
        public void BinaryRead(BinaryReader r)
        {
            dict.Clear();
            var count = r.ReadUInt32();
            for (var i = 0; i < count; i++)
            {
                var item = new {{ELEMENT_NAME}}();
                item.BinaryRead(r);
                dict[item.id] = item;
            }
        }

		/// <inheritdoc cref="IBinarySupport.BinaryWriter"/>
        public void BinaryWrite(BinaryWriter w)
        {
            w.Write(Convert.ToUInt32(dict.Count));
            foreach (var (_, item) in dict)
            {
                item.BinaryWrite(w);
            }
        }
    }

    public partial class {{ELEMENT_NAME}} : IBinarySupport
    {
		/// <inheritdoc cref="IBinarySupport.BinaryRead"/>
        public void BinaryRead(BinaryReader r)
        {
{%- for field in CLASS_FIELDS %}
            {{ field.name }} = {{ field.reader }};
{%- endfor %}
        }

		/// <inheritdoc cref="IBinarySupport.BinaryWriter"/>
        public void BinaryWrite(BinaryWriter w)
        {
{%- for field in CLASS_FIELDS %}
            w.Write({{ field.name }});
{%- endfor %}
        }
    }
{%- endif %}
{% if SUPPORT_CLONE %}
    public partial class {{TABLE_NAME}} : ICloneable
    {
        public object Clone()
        {
            return MemberwiseClone();
        }
    }

    partial class {{ELEMENT_NAME}} : ICloneable
    {
        public object Clone()
        {
            return MemberwiseClone();
        }
    }
{%- endif %}
}