using System.IO;

namespace __NAMESPACE__
{
    public interface IBinarySupport
    {
        /// <summary>
        /// 读取二进制文件
        /// </summary>
        /// <param name="r"></param>
        public abstract void BinaryRead(BinaryReader r);

        /// <summary>
        /// 写入二进制文件
        /// </summary>
        /// <param name="w"></param>
        public abstract void BinaryWrite(BinaryWriter w);
    }

    public interface IXmlSupport
    {
        /// <summary>
        /// 读取 XML
        /// </summary>
        /// <param name="r"></param>
        public abstract void XmlRead(BinaryReader r);
        /// <summary>
        /// 写入 XML
        /// </summary>
        /// <param name="w"></param>
        public abstract void XmlWrite(BinaryWriter w);
    }
}