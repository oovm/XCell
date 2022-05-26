    [DataContract]
    public partial class __TABLE_NAME__
    {
        [DataMember] public readonly Dictionary<int, __ELEMENT_NAME__> dict = new();

        /// <summary>
        /// 获取 TestElement
        /// </summary>
        /// <param name="id"></param>
        /// <returns></returns>
        [CanBeNull]
        public __ELEMENT_NAME__ GetElement(int id)
        {
            return dict.TryGetValue(id, out var item) ? item : null;
        }
    }

    [DataContract]
    public partial class __ELEMENT_NAME__
    {
        /// <summary>
        ///
        /// </summary>
        [DataMember] public int id;

        /// <summary>
        ///
        /// </summary>
        [DataMember] public bool testBool;

        /// <summary>
        ///
        /// </summary>
        [DataMember] public byte testU8;

        /// <summary>
        ///
        /// </summary>
        [DataMember] public double testU16;
    }