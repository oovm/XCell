using System;
using System.Runtime.Serialization;

namespace {{ NAMESPACE }}
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