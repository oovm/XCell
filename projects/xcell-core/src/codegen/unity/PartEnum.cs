// 代码生成, 修改无效!
// 当前版本: v{{ VERSION }}
// 查看更新: https://github.com/oovm/XCell
using System;
using System.Runtime.Serialization;

namespace {{ config.namespace }}
{
    [DataContract]
    public enum {{ TABLE_NAME }} : long
    {

        SSR = 1,

        SSR = 1,
        SR = 2,
    }

    public static class {{ TABLE_NAME }}Extension
    {
        public static __M_TYPE__ __M_NAME__(this __TABLE_NAME__ self)
        {
            return self switch
            {
                {{ TABLE_NAME }}.__FIELD__ => __VALUE__,
                _ => throw new ArgumentOutOfRangeException(nameof(self), self, null)
            };
        }

        public static __M_TYPE__ __M_NAME__(this __TABLE_NAME__ self)
        {
            return self switch
            {
                __TABLE_NAME__.__FIELD__ => __VALUE__,
                _ => throw new ArgumentOutOfRangeException(nameof(self), self, null)
            };
        }

    }
}