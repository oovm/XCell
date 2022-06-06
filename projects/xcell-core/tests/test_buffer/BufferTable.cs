// 代码生成, 修改无效!
// 当前版本: v0.0.0
// 查看更新: https://github.com/oovm/XCell
// ReSharper disable RedundantDefaultMemberInitializer, RedundantUsingDirective
// ReSharper disable CheckNamespace
// ReSharper disable ArrangeObjectCreationWhenTypeEvident

using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization;
using System.Text;
using JetBrains.Annotations;
using UnityEngine;

namespace DataTable.Generated
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
        /// Item 类型
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public int id = 0;

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
        /// 颜色
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public Color32 test1 = new Color32(0, 0, 0, 255);

        /// <summary>
        /// 无符号 8 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public sbyte test2 = 0;

        /// <summary>
        /// 无符号 16 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public ushort test3 = 0;

        /// <summary>
        /// 无符号 32 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public uint test4 = 0;

        /// <summary>
        /// 无符号 64 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public ulong test5 = 0;

        /// <summary>
        /// 有符号 8 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public byte test6 = 0;

        /// <summary>
        /// 有符号 16 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public short test7 = 0;

        /// <summary>
        /// 有符号 32 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public int test8 = 0;

        /// <summary>
        /// 有符号 64 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public long test9 = 0;

        /// <summary>
        /// 32 位浮点数
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public float test10 = 0;

        /// <summary>
        /// 64 位浮点数
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public double test11 = 0;

        /// <summary>
        /// 128 位高精度
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public decimal test12 = 0;

    }

    public partial class BuffTable : IBinarySupport
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
                var item = new BuffElement();
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

    public partial class BuffElement : IBinarySupport
    {
		/// <inheritdoc cref="IBinarySupport.BinaryRead"/>
        public void BinaryRead(BinaryReader r)
        {
            id = r.ReadInt32();
            name = r.ReadString();
            test0 = r.ReadBoolean();
            test1 = new Color32(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte());
            test2 = r.ReadSByte();
            test3 = r.ReadUInt16();
            test4 = r.ReadUInt32();
            test5 = r.ReadUInt64();
            test6 = r.ReadByte();
            test7 = r.ReadInt16();
            test8 = r.ReadInt32();
            test9 = r.ReadInt64();
            test10 = r.ReadSingle();
            test11 = r.ReadDouble();
            test12 = r.ReadDecimal();
        }

		/// <inheritdoc cref="IBinarySupport.BinaryWrite"/>
        public void BinaryWrite(BinaryWriter w)
        {
            w.Write(id);
            w.Write(name);
            w.Write(test0);
            w.Write(test1.r);
            w.Write(test1.g);
            w.Write(test1.b);
            w.Write(test1.a);
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
            w.Write(test12);
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