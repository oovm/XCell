// 代码生成, 修改无效!
// 当前版本: v0.0.0
// 查看更新: https://github.com/oovm/XCell
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

namespace DataTable.Generated
{
    [DataContract, Serializable]
    public enum RarityType : ulong
    {
        SSR = 1,
        SR = 2,
    }

    public static class RarityTypeExtension
    {
        public static ulong GetId(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.id;
        }
        public static string GetName(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.name;
        }
        public static bool GetTest0(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test0;
        }
        public static Color32 GetTest1(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test1;
        }
        public static sbyte GetTest2(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test2;
        }
        public static ushort GetTest3(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test3;
        }
        public static uint GetTest4(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test4;
        }
        public static ulong GetTest5(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test5;
        }
        public static byte GetTest6(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test6;
        }
        public static short GetTest7(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test7;
        }
        public static int GetTest8(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test8;
        }
        public static long GetTest9(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test9;
        }
        public static float GetTest10(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test10;
        }
        public static double GetTest11(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test11;
        }
        public static decimal GetTest12(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test12;
        }
        public static DateTime GetTest13(this RarityType self)
        {
            return DataTableManager.Instance.RarityTypeTable.GetElement((ulong)self)!.test13;
        }
    }
    [DataContract, Serializable]
    public partial class RarityTypeTable
    {
        [DataMember]
        public readonly Dictionary<ulong, RarityTypeElement> dict = new();

        [CanBeNull]
        public RarityTypeElement GetElement(ulong id)
        {
            return dict.TryGetValue(id, out var item) ? item : null;
        }
    }

    [DataContract, Serializable]
    public partial class RarityTypeElement
    {
        /// <summary>
        /// 编号
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public ulong id = 0;

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

        /// <summary>
        /// 日期
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public DateTime test13 = new DateTime(2022, 11, 12, 12, 49, 10);

    }

    public partial class RarityTypeTable : IBinarySupport
    {
        public RarityTypeTable()
        {
            BinaryRead("Assets/Tables/Binary/RarityTypeTable.binary");
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
                var item = new RarityTypeElement();
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

    public partial class RarityTypeElement : IBinarySupport
    {
		/// <inheritdoc cref="IBinarySupport.BinaryRead"/>
        public void BinaryRead(BinaryReader r)
        {
            id = r.ReadUInt64();
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
            test13 = new DateTime(r.ReadInt64(), DateTimeKind.Utc);
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
            w.Write(test13.Ticks);
        }
    }

    public partial class RarityTypeTable : ICloneable
    {
        public object Clone()
        {
            return MemberwiseClone();
        }
    }

    partial class RarityTypeElement : ICloneable
    {
        public object Clone()
        {
            return MemberwiseClone();
        }
    }
}