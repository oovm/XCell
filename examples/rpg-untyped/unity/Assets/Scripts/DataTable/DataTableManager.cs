using System;
using System.Collections.Generic;
using System.IO;
using UnityEngine;

namespace DataTable
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
            ItemTable.Load();
            MonsterTypeTable.Load();
            MonstersTable.Load();
            PlayerLevelsTable.Load();
            SkillsTable.Load();
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

        /// <summary>
        /// 加载二进制文件
        /// </summary>
        /// <param name="path">文件路径</param>
        /// <returns>字节数组</returns>
        public static byte[] LoadBinaryFile(string path)
        {
            TextAsset asset = Resources.Load<TextAsset>(path);
            if (asset == null)
            {
                Debug.LogError($"Failed to load binary file: {path}");
                return null;
            }
            return asset.bytes;
        }
    }
}
