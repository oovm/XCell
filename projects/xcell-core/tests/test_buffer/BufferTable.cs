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
        [DataMember]
        public int id = 0;

        /// <summary>
        /// 名字
        /// </summary>
        [DataMember]
        public string name = "";

        /// <summary>
        /// Buff组
        /// </summary>
        [DataMember]
        public int groupId = 0;

        /// <summary>
        /// 描述
        /// </summary>
        [DataMember]
        public string desc = "";

        /// <summary>
        /// 图标
        /// </summary>
        [DataMember]
        public string icon = "";

        /// <summary>
        /// 最大堆叠, 0 表示无限
        /// </summary>
        [DataMember]
        public uint max = 0;

        /// <summary>
        /// behavior Id
        /// </summary>
        [DataMember]
        public stringId actionId = ;
    }
