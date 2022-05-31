using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization;
using JetBrains.Annotations;

// ReSharper disable RedundantDefaultMemberInitializer
namespace DataTable
{
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
        /// Buff ID
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public int id = 0;

        /// <summary>
        /// 名字
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public string name = "";

        /// <summary>
        /// Buff组
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public int groupId = 0;

        /// <summary>
        /// 描述
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public string desc = "";

        /// <summary>
        /// 图标
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public string icon = "";

        /// <summary>
        /// 最大堆叠, 0 表示无限
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public uint max = 0;

        /// <summary>
        /// behavior Id
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public stringId actionId = ;
    }
    public partial class BuffTable : IBinarySupport
    {
        public void BinaryRead(BinaryReader r)
        {
            dict.Clear();
            var count = r.Read();
            for (var i = 0; i < count; i++)
            {
                var item = new TestElement();
                item.BinaryRead(r);
                dict[item.id] = item;
            }
        }
    
        public void BinaryWrite(BinaryWriter w)
        {
            w.Write(dict.Count);
            foreach (var (_, item) in dict)
            {
                item.BinaryWrite(w);
            }
        }
    }
    
    partial class TestElement : IBinarySupport
    {
        public void BinaryRead(BinaryReader r)
        {
        id = r.Read();

        name = r.Read();

        groupId = r.Read();

        desc = r.Read();

        icon = r.Read();

        max = r.Read();

        actionId = r.Read();
        }
    
        public void BinaryWrite(BinaryWriter w)
        {
        w.Write(id);

        w.Write(name);

        w.Write(groupId);

        w.Write(desc);

        w.Write(icon);

        w.Write(max);

        w.Write(actionId);
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
