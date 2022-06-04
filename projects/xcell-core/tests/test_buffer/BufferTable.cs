// 代码生成, 修改无效!
// 当前版本: v0.0.0
// 查看更新: https://github.com/oovm/XCell

using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization;
using JetBrains.Annotations;

// ReSharper disable RedundantDefaultMemberInitializer
namespace DataTable
{
    [DataContract]
    public partial class BuffTable
    {
        [DataMember] public readonly Dictionary<int, BuffElement> dict = new();

        [CanBeNull]
        public BuffElement GetElement(int id)
        {
            return dict.TryGetValue(id, out var item) ? item : null;
        }
    }

    [DataContract]
    public partial class BuffElement
    {
        /// <summary>
        /// Buff 类型
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public RarityType enum;
        /// <summary>
        /// 本地化
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public string name = "";

        /// <summary>
        /// 布尔
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public bool test0 = false;

        /// <summary>
        /// 无符号 8 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public sbyte test1 = 0;

        /// <summary>
        /// 无符号 16 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public ushort test2 = 0;

        /// <summary>
        /// 无符号 32 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public uint test3 = 0;

        /// <summary>
        /// 无符号 64 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public ulong test4 = 0;

        /// <summary>
        /// 有符号 8 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public byte test5 = 0;

        /// <summary>
        /// 有符号 16 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public short test6 = 0;

        /// <summary>
        /// 有符号 32 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public int test7 = 0;

        /// <summary>
        /// 有符号 64 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public long test8 = 0;

        /// <summary>
        /// f32
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public float test9 = 0;

        /// <summary>
        /// f64
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public double test10 = 0;

        /// <summary>
        /// d128
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public d128 test11;
    }

    public partial class BuffTable : IBinarySupport
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
                var item = new BuffElement();
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

    public partial class BuffElement : IBinarySupport
    {
        /// <summary>
        /// 以小端序 (LittleEndian) 读取二进制数据
        /// </summary>
        /// <param name="r">二进制读取流</param>
        public void BinaryRead(BinaryReader r)
        {
            enum = ;
            name = r.ReadString();
            test0 = r.ReadBoolean();
            test1 = r.ReadSByte();
            test2 = r.ReadUInt16();
            test3 = r.ReadUInt32();
            test4 = r.ReadUInt64();
            test5 = r.ReadByte();
            test6 = r.ReadInt16();
            test7 = r.ReadInt32();
            test8 = r.ReadInt64();
            test9 = r.ReadSingle();
            test10 = r.ReadDouble();
            test11 = ;
        }
        /// <summary>
        /// 以小端序 (LittleEndian) 写入二进制数据
        /// </summary>
        /// <param name="w">二进制写入流</param>
        public void BinaryWrite(BinaryWriter w)
        {
            w.Write(enum);
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
            w.Write(test10);
            w.Write(test11);
        }
    }

    public partial class BuffTable : ICloneable
    {
        public object Clone()
        {
            return MemberwiseClone();
        }
    }

    partial class BuffElement : ICloneable
    {
        public object Clone()
        {
            return MemberwiseClone();
        }
    }
}