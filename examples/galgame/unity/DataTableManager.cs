// Unity generated file

using System;
using System.Collections.Generic;
using System.IO;
using UnityEngine;

namespace {{ namespace }}
{
    /// <summary>
    /// 数据表管理器
    /// 负责加载和管理所有数据表
    /// </summary>
    public class DataTableManager
    {
        private static DataTableManager instance;
        private Dictionary<string, object> tables = new Dictionary<string, object>();

        /// <summary>
        /// 获取单例实例
        /// </summary>
        public static DataTableManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new DataTableManager();
                }
                return instance;
            }
        }

        /// <summary>
        /// 加载所有数据表
        /// </summary>
        public void LoadAllTables()
        {
{% for table in tables %}
            {{ table.table_name }}.Load();
{% endfor %}
        }

        /// <summary>
        /// 存储数据表
        /// </summary>
        /// <typeparam name="T">数据表类型</typeparam>
        /// <param name="tableName">表名</param>
        /// <param name="table">数据表实例</param>
        public void SetTable<T>(string tableName, T table)
        {
            tables[tableName] = table;
        }

        /// <summary>
        /// 获取数据表
        /// </summary>
        /// <typeparam name="T">数据表类型</typeparam>
        /// <param name="tableName">表名</param>
        /// <returns>数据表实例</returns>
        public T GetTable<T>(string tableName)
        {
            if (tables.TryGetValue(tableName, out var table))
            {
                return (T)table;
            }
            return default;
        }
    }
}