using System;
using System.Collections.Generic;
using System.IO;
using UnityEngine;

namespace DataTable
{
    /// <summary>
    /// 怪物类型表数据结构
    /// </summary>
    public class MonsterType
    {
        /// <summary>
        /// 怪物类型变体
        /// </summary>
        public string Variant { get; set; }
        /// <summary>
        /// 怪物类型ID
        /// </summary>
        public string Id { get; set; }
        /// <summary>
        /// 怪物类型名称
        /// </summary>
        public string Name { get; set; }
        /// <summary>
        /// 怪物类型描述
        /// </summary>
        public string Description { get; set; }
    }

    /// <summary>
    /// 怪物类型表加载器
    /// </summary>
    public static class MonsterTypeTable
    {
        private static Dictionary<string, MonsterType> monsterTypes = new Dictionary<string, MonsterType>();

        /// <summary>
        /// 加载怪物类型表数据
        /// </summary>
        public static void Load()
        {
            // 这里可以从二进制文件或JSON文件加载数据
            // 暂时使用硬编码数据作为示例
            LoadSampleData();
            
            // 存储到 DataTableManager
            DataTableManager.Instance.SetTable("MonsterType", monsterTypes);
        }

        /// <summary>
        /// 加载示例数据
        /// </summary>
        private static void LoadSampleData()
        {
            monsterTypes.Clear();
            
            // 示例数据
            monsterTypes.Add("2", new MonsterType
            {
                Variant = "Undead",
                Id = "2",
                Name = "undead",
                Description = "undead 怪物"
            });
            
            monsterTypes.Add("3", new MonsterType
            {
                Variant = "Beast",
                Id = "3",
                Name = "野兽",
                Description = "野兽"
            });
            
            monsterTypes.Add("4", new MonsterType
            {
                Variant = "Humanoid",
                Id = "4",
                Name = "人形",
                Description = "人形生物"
            });
        }

        /// <summary>
        /// 根据ID获取怪物类型
        /// </summary>
        /// <param name="id">怪物类型ID</param>
        /// <returns>怪物类型实例</returns>
        public static MonsterType GetMonsterTypeById(string id)
        {
            if (monsterTypes.TryGetValue(id, out var type))
            {
                return type;
            }
            return null;
        }

        /// <summary>
        /// 根据变体获取怪物类型
        /// </summary>
        /// <param name="variant">怪物类型变体</param>
        /// <returns>怪物类型实例</returns>
        public static MonsterType GetMonsterTypeByVariant(string variant)
        {
            foreach (var type in monsterTypes.Values)
            {
                if (type.Variant == variant)
                {
                    return type;
                }
            }
            return null;
        }

        /// <summary>
        /// 获取所有怪物类型
        /// </summary>
        /// <returns>怪物类型列表</returns>
        public static List<MonsterType> GetAllMonsterTypes()
        {
            return new List<MonsterType>(monsterTypes.Values);
        }
    }
}
