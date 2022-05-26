[DataContract]
public partial class __TABLE_NAME__
{
    [DataMember] public readonly Dictionary<int, __ELEMENT_NAME__> dict = new();

    [CanBeNull]
    public __ELEMENT_NAME__ __ELEMENT_GETTER__(int id)
    {
        return dict.TryGetValue(id, out var item) ? item : null;
    }
}

[DataContract]
public partial class __ELEMENT_NAME__
{