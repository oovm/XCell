// 代码生成, 修改无效!
// 当前版本: v{{ VERSION }}
// 查看更新: https://github.com/oovm/XCell
using System.IO;

namespace {{ NAMESPACE }}
{
    public interface IBinarySupport
    {
        /// <summary>
        /// 以小端序 (LittleEndian) 读取二进制数据
        /// </summary>
        /// <param name="r">二进制读取流</param>
        public abstract void BinaryRead(BinaryReader r);

        /// <summary>
        /// 以小端序 (LittleEndian) 写入二进制数据
        /// </summary>
        /// <param name="w">二进制写入流</param>
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