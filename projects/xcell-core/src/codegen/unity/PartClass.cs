// Generated File

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
        /// <summary>
        /// Buff ID
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public int id = 0;

        /// <summary>
        /// 名字
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public string name = "";

        /// <summary>
        /// boolean
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public bool test0 = false;

        /// <summary>
        /// u8
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public sbyte test1 = 0;

        /// <summary>
        /// u16
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public ushort test2 = 0;

        /// <summary>
        /// u32
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public uint test3 = 0;

        /// <summary>
        /// u64
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public ulong test4 = 0;

        /// <summary>
        /// i8
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public byte test5 = 0;

        /// <summary>
        /// i16
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public short test6 = 0;

        /// <summary>
        /// i32
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public int test7 = 0;

        /// <summary>
        /// i64
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public long test8 = 0;

        /// <summary>
        /// f32
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public float test9 = 0;

        /// <summary>
        /// f64
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember] public ArchiveType test10 = 0;
    }
{% if SUPPORT_BINARY %}
    public partial class {{TABLE_NAME}} : IBinarySupport
    {
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
        public void BinaryRead(BinaryReader r)
        {
            id = r.ReadInt32();
            name = r.ReadString();
            test0 = r.ReadBoolean();
            test1 = r.ReadSByte();
            test2 = r.ReadUInt16();
            test3 = r.ReadUInt32();
            test4 = r.ReadUInt64();
            test5 = r.ReadByte();
            test6 = r.ReadInt16();
            test7 = r.ReadInt32();
            test8 = r.ReadUInt16();
            test9 = r.ReadSingle();
            test10 = (ArchiveType) r.ReadInt64();
        }

        public void BinaryWrite(BinaryWriter w)
        {
            w.Write(id);
            w.Write(name);
            w.Write(test0);
            w.Write(test1);
            w.Write(test2);
            w.Write(test3);
            w.Write(test4);
            w.Write(test5);
            w.Write(test6);
            w.Write(test7);
            w.Write(test8);
            w.Write(test9);
            w.Write((long) test10);
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