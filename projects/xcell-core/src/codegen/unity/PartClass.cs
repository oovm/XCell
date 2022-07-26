// 代码生成, 修改无效!
// ReSharper disable RedundantDefaultMemberInitializer, RedundantUsingDirective
// ReSharper disable ArrangeObjectCreationWhenTypeEvident
// ReSharper disable CheckNamespace

using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization;
using System.Text;
using JetBrains.Annotations;
using UnityEngine;

namespace {{ config.namespace }}
{
{%- if enumerate %}
    [DataContract, Serializable]
    public enum {{CLASS_NAME}} : {{ ID_TYPE }}
    {
        SSR = 1,
        SR = 2,
    }

    public static class {{CLASS_NAME}}Extension
    {
{%- for field in CLASS_FIELDS %}
        /// <inheritdoc cref="{{ELEMENT_NAME}}.{{ field.name }}"/>
        public static {{ field.typing }} {{ field.getter }}(this {{CLASS_NAME}} self)
        {
            return {{ config.manager_name }}.Instance.{{ TABLE_NAME }}.{{ ELEMENT_GETTER }}(({{ ID_TYPE }}) self)!.{{ field.name }};
        }
{%- endfor %}
    }
{%- endif %}
    [DataContract, Serializable]
    public partial class {{ TABLE_NAME }}
    {
        [DataMember]
        public readonly Dictionary<{{ ID_TYPE }}, {{ELEMENT_NAME}}> dict = new();

        [CanBeNull]
        public {{ELEMENT_NAME}} {{ ELEMENT_GETTER }}({{ ID_TYPE }} id)
        {
            return dict.TryGetValue(id, out var item) ? item : null;
        }
    }

    [DataContract, Serializable]
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
{% if config.support_binary %}
    public partial class {{TABLE_NAME}} : IBinarySupport
    {
        public {{TABLE_NAME}}()
        {
            BinaryRead("Assets/{{ config.folder_binary }}/{{ TABLE_NAME }}.binary");
        }
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

		/// <inheritdoc cref="IBinarySupport.BinaryWrite"/>
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

		/// <inheritdoc cref="IBinarySupport.BinaryWrite"/>
        public void BinaryWrite(BinaryWriter w)
        {
{%- for field in CLASS_FIELDS %}
{%- for line in field.writer %}
            {{ line }}
{%- endfor %}
{%- endfor %}
        }
    }
{%- endif %}
{% if config.support_clone %}
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