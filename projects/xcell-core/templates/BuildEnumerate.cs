// 代码生成, 修改无效! (XCell {{ compiler_version }})
// ReSharper disable RedundantNullableDirective, RedundantUsingDirective
// ReSharper disable EnumUnderlyingTypeIsInt
// ReSharper disable CheckNamespace

#nullable enable
using System;
using System.Runtime.Serialization;
using UnityEngine;

namespace {{ config.namespace }}
{
    [DataContract, Serializable]
    public enum {{ class_name }}: {{ id_type }}
    {
{%- for field in enumerate_ids %}
    {%- for line in field.document %}
        /// {{ line }}
    {%- endfor %}
        [EnumMember] {{ field.key }} = {{ field.value }},
{%- endfor %}
    }

    public static class {{ class_name }}Extension
    {
{%- for field in enumerate_fields %}
    {%- for line in field.document %}
        /// {{ line }}
    {%- endfor %}
        public static {{ field.typing }} {{ field.getter }}(this {{ class_name }} self)
        {
            return self switch
            {
            {%- for line in field.switch %}
                {{ class_name }}.{{ line.key }} => {{ line.value }},
            {%- endfor %}
                _ => throw new ArgumentOutOfRangeException(nameof(self), self, null)
            };
        }
{% endfor %}
    }
}