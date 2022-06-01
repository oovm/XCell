using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization;
using JetBrains.Annotations;

// ReSharper disable RedundantDefaultMemberInitializer
namespace DataTable
{    [DataContract]
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
        /// boolean
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public bool test0 = false;

        /// <summary>
        /// u8
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public sbyte test1 = 0;

        /// <summary>
        /// u16
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public ushort test2 = 0;

        /// <summary>
        /// u32
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public uint test3 = 0;

        /// <summary>
        /// u64
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public ulong test4 = 0;

        /// <summary>
        /// i8
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public byte test5 = 0;

        /// <summary>
        /// i16
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public short test6 = 0;

        /// <summary>
        /// i32
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public int test7 = 0;

        /// <summary>
        /// i64
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
        public long test8 = 0;

        /// <summary>
        /// f32
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        [DataMember]
