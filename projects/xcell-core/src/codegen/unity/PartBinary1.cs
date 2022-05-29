public partial class __TABLE_NAME__ : IBinarySupport
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