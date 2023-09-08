// 代码生成, 修改无效! (XCell {{ compiler_version }})
// ReSharper disable RedundantDefaultMemberInitializer, RedundantUsingDirective
// ReSharper disable ArrangeObjectCreationWhenTypeEvident, ArrangeObjectCreationWhenTypeNotEvident
// ReSharper disable EnumUnderlyingTypeIsInt
// ReSharper disable CheckNamespace

#nullable enable
using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization;
using System.Text;
using UnityEngine;
{%- if config.addressable %}
using UnityEngine.AddressableAssets;
{% endif %}

namespace {{ config.namespace }}
{
    [DataContract, Serializable]
    public partial class {{ table_name }}: IReadOnlyDictionary<{{ id_type }}, {{ class_name }}Element>
    {
        [NonSerialized]
        private readonly Dictionary<{{ id_type }}, {{ class_name }}Element> dict = new();
        [DataMember]
        public readonly List<{{ class_name }}Element> elements = new();
        public int Count => dict.Count;
        public IEnumerable<{{ id_type }}> Keys => dict.Keys;
        public IEnumerable<{{ class_name }}Element> Values => dict.Values;
        public {{ class_name }}Element this[{{ id_type }} key] => dict[key];
        IEnumerator IEnumerable.GetEnumerator()
        {
            return dict.GetEnumerator();
        }
        public IEnumerator<KeyValuePair<{{ id_type }}, {{ class_name }}Element>> GetEnumerator()
        {
            return dict.GetEnumerator();
        }
        public bool ContainsKey({{ id_type }} key)
        {
            return dict.ContainsKey(key);
        }
        public bool TryGetValue({{ id_type }} key, out {{ class_name }}Element value)
        {
            return dict.TryGetValue(key, out value);
        }
        public {{ class_name }}Element? GetValue(uint key)
        {
            return dict.TryGetValue(key, out var value) ? value : null;
        }
    }

    [DataContract, Serializable]
    public partial class {{ class_name }}Element
    {
{%- for field in class_fields %}
    {%- for line in field.document %}
        /// {{line}}
    {%- endfor %}
        [DataMember]
    {%- if field.has_default %}
        {{field.access}}{{field.typing}} {{field.name}} = {{field.default}};
    {% else %}
        {{field.access}}{{field.typing}} {{field.name}};
    {%- endif %}
{%- endfor %}
    }
{% if config.binary.enable %}
    public partial class {{ table_name }} : IBinarySupport
    {
        public {{ table_name }}()
        {
    {%- if config.addressable %}
            BinaryRead("{{ config.binary.addressable }}/{{ table_name }}.bytes");
    {%- else %}
            BinaryRead("{{ config.binary.output }}/{{ table_name }}.bytes");
    {%- endif %}
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
        public async void BinaryRead(string path)
        {
            var handle = await Addressables.LoadAssetAsync<TextAsset>(path).Task;
            using var stream = new MemoryStream(handle.bytes);
            using var reader = new BinaryReader(stream, Encoding.UTF8, false);
            BinaryRead(reader);
        }

		/// <inheritdoc cref="IBinarySupport.BinaryRead"/>
        public void BinaryRead(BinaryReader r)
        {
            elements.Clear();
            dict.Clear();
            var count = r.ReadUInt32();
            for (var i = 0; i < count; i++)
            {
                var item = new {{ class_name }}Element();
                item.BinaryRead(r);
                elements.Add(item);
                dict[item.{{ key_name }}] = item;
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
{% if config.support_clone %}
        /// <inheritdoc cref="ICloneable.Clone"/>
        public {{ table_name }} Clone()
        {
            return ({{ table_name }}) MemberwiseClone();
        }
{%- endif %}
    }

    public partial class {{ class_name }}Element : IBinarySupport
    {
		/// <inheritdoc cref="IBinarySupport.BinaryRead"/>
        public void BinaryRead(BinaryReader r)
        {
{%- for field in class_fields %}
    {%- if field.reader.is_vector %}
            var {{ field.reader.field }}Count = r.ReadUInt32();
            {{ field.reader.field }} = new((int) {{ field.reader.field }}Count);
            for (var i = 0; i < {{ field.reader.field }}Count; i++)
            {
                {{ field.reader.field }}.Add({{ field.reader.function }});
            }
    {%- else %}
            {{ field.reader.field }} = {{ field.reader.function }};
    {%- endif %}
{%- endfor %}
        }

		/// <inheritdoc cref="IBinarySupport.BinaryWrite"/>
        public void BinaryWrite(BinaryWriter w)
        {
{%- for field in class_fields %}
	{%- if field.writer.is_vector %}
            w.Write((uint) {{ field.writer.field }}.Count);
            foreach (var {{ field.writer.field }}Item in {{ field.writer.field }})
            {
		{%- for property in field.writer.properties %}
                w.Write({{ field.writer.field }}Item{{ property }});
		{%- endfor %}
            }
	{%- else %}
		{%- for property in field.writer.properties %}
            w.Write({{ field.writer.cast }}{{ field.writer.field }}{{ property }});
		{%- endfor %}
	{%- endif %}
{%- endfor %}
        }
{% if config.support_clone %}
        /// <inheritdoc cref="ICloneable.Clone"/>
        public {{ class_name }}Element Clone()
        {
            return ({{ class_name }}Element) MemberwiseClone();
        }
{%- endif %}
    }
{%- endif %}
}