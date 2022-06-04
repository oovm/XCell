// 代码生成, 修改无效!
// 当前版本: v{{ VERSION }}
// 查看更新: https://github.com/oovm/XCell

using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization;
using JetBrains.Annotations;

// ReSharper disable RedundantDefaultMemberInitializer
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
        /// 以小端序 (LittleEndian) 读取二进制数据
        /// </summary>
        /// <param name="r">二进制读取流</param>
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
        /// <summary>
        /// 以小端序 (LittleEndian) 写入二进制数据
        /// </summary>
        /// <param name="w">二进制写入流</param>
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
        /// <summary>
        /// 以小端序 (LittleEndian) 读取二进制数据
        /// </summary>
        /// <param name="r">二进制读取流</param>
        public void BinaryRead(BinaryReader r)
        {
{%- for field in CLASS_FIELDS %}
            {{ field.name }} = {{ field.reader }};
{%- endfor %}
        }
        /// <summary>
        /// 以小端序 (LittleEndian) 写入二进制数据
        /// </summary>
        /// <param name="w">二进制写入流</param>
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