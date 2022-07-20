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
    public partial class BuffTable
    {
        [DataMember]
        public readonly Dictionary<int, BuffElement> dict = new();

        [CanBeNull]
        public BuffElement GetElement(int id)
        {
            return dict.TryGetValue(id, out var item) ? item : null;
        }
    }

    [DataContract, Serializable]
    public partial class BuffElement
    {
        /// <summary>
        /// Item 类型
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public int id;
        /// <summary>
        /// 本地化
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public string name;
        /// <summary>
        /// 布尔
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public bool test0;
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
        public sbyte test2;
        /// <summary>
        /// 无符号 16 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public ushort test3;
        /// <summary>
        /// 无符号 32 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public uint test4;
        /// <summary>
        /// 无符号 64 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public ulong test5;
        /// <summary>
        /// 有符号 8 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public byte test6;
        /// <summary>
        /// 有符号 16 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public short test7;
        /// <summary>
        /// 有符号 32 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public int test8;
        /// <summary>
        /// 有符号 64 位
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public long test9;
        /// <summary>
        /// 32 位浮点数
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public float test10;
        /// <summary>
        /// 64 位浮点数
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public float test11;
        /// <summary>
        /// 128 位高精度
        /// </summary>
        /// <remarks>
        /// </remarks>
        [DataMember]
        public float test12;
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