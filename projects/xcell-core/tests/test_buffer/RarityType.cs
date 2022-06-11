// 代码生成, 修改无效!
// 当前版本: v0.0.0
// 查看更新: https://github.com/oovm/XCell
// ReSharper disable RedundantDefaultMemberInitializer, RedundantUsingDirective
// ReSharper disable ArrangeTrailingCommaInMultilineLists
// ReSharper disable CheckNamespace

using System;
using System.Collections.Generic;
using System.Runtime.Serialization;
using DataTable.Generated;
using JetBrains.Annotations;

namespace DataTable.Generated
{
    [DataContract, Serializable]
    public enum RarityType : long
    {
        SSR = 1,
        SR = 2,
    }

    public static class RarityTypeExtension
    {
        public static string GetName(this ArchiveType self)
        {
            return DataTableManager.Instance.ArchiveTypeTable.GetElement(self)!.name;
        }
    }

    [DataContract]
    public partial class RarityTypeTable
    {
        [DataMember] public readonly Dictionary<int, RarityTypeElement> dict = new();

        [CanBeNull]
        public RarityTypeElement GetElement(int id)
        {
            return dict.TryGetValue(id, out var item) ? item : null;
        }
    }

    [DataContract]
    public partial class RarityTypeElement
    {
        /// <summary>
        /// Item 类型
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public RarityType enum;
        /// <summary>
        /// 编号
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public uint id = 0;

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
        public uint test2 = 0;

        /// <summary>
        /// 无符号 16 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public uint test3 = 0;

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
        public uint test5 = 0;

        /// <summary>
        /// 有符号 8 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public uint test6 = 0;

        /// <summary>
        /// 有符号 16 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public uint test7 = 0;

        /// <summary>
        /// 有符号 32 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public uint test8 = 0;

        /// <summary>
        /// 有符号 64 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public uint test9 = 0;

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

    public partial class RarityTypeTable : IBinarySupport
    {
        public BuffTable()
        {
            BinaryRead("Assets/Tables/Binary/RarityTypeTable.xcell");
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
            enum = ;
            id = r.ReadUInt32();
            name = r.ReadString();
            test0 = r.ReadBoolean();
            test1 = new Color32(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte());
            test2 = r.ReadUInt32();
            test3 = r.ReadUInt32();
            test4 = r.ReadUInt32();
            test5 = r.ReadUInt32();
            test6 = r.ReadUInt32();
            test7 = r.ReadUInt32();
            test8 = r.ReadUInt32();
            test9 = r.ReadUInt32();
            test10 = r.ReadSingle();
            test11 = r.ReadDouble();
            test12 = r.ReadDecimal();
        }

		/// <inheritdoc cref="IBinarySupport.BinaryWrite"/>
        public void BinaryWrite(BinaryWriter w)
        {
            w.Write(enum);
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