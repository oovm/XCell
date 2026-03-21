using System;
using System.Collections.Generic;
using System.IO;
using UnityEngine;

namespace DataTable
{
    /// <summary>
    /// 物品表数据结构
    /// </summary>
    public class Item
    {
        /// <summary>
        /// 物品ID
        /// </summary>
        public string Id { get; set; }
        /// <summary>
        /// 物品名称
        /// </summary>
        public string Name { get; set; }
        /// <summary>
        /// 物品类型
        /// </summary>
        public string Type { get; set; }
        /// <summary>
        /// 物品等级
        /// </summary>
        public string Level { get; set; }
        /// <summary>
        /// 攻击力
        /// </summary>
        public string Attack { get; set; }
        /// <summary>
        /// 防御力
        /// </summary>
        public string Defense { get; set; }
        /// <summary>
        /// 物品描述
        /// </summary>
        public string Description { get; set; }
    }

    /// <summary>
    /// 物品表加载器
    /// </summary>
    public static class ItemTable
    {
        private static Dictionary<string, Item> items = new Dictionary<string, Item>();

        /// <summary>
        /// 加载物品表数据
        /// </summary>
        public static void Load()
        {
            // 这里可以从二进制文件或JSON文件加载数据
            // 暂时使用硬编码数据作为示例
            LoadSampleData();
            
            // 存储到 DataTableManager
            DataTableManager.Instance.SetTable("Item", items);
        }

        /// <summary>
        /// 加载示例数据
        /// </summary>
        private static void LoadSampleData()
        {
            items.Clear();
            
            // 示例数据
            items.Add("2", new Item
            {
                Id = "2",
                Name = "木盾",
                Type = "armor",
                Level = "1",
                Attack = "0",
                Defense = "5",
                Description = "一个简单的木盾"
            });
            
            items.Add("3", new Item
            {
                Id = "3",
                Name = "生命药水",
                Type = "consumable",
                Level = "1",
                Attack = "0",
                Defense = "0",
                Description = "恢复50点生命值"
            });
            
            items.Add("4", new Item
            {
                Id = "4",
                Name = "锁子甲",
                Type = "armor",
                Level = "2",
                Attack = "0",
                Defense = "10",
                Description = "一件锁子甲"
            });
        }

        /// <summary>
        /// 根据ID获取物品
        /// </summary>
        /// <param name="id">物品ID</param>
        /// <returns>物品实例</returns>
        public static Item GetItemById(string id)
        {
            if (items.TryGetValue(id, out var item))
            {
                return item;
            }
            return null;
        }

        /// <summary>
        /// 获取所有物品
        /// </summary>
        /// <returns>物品列表</returns>
        public static List<Item> GetAllItems()
        {
            return new List<Item>(items.Values);
        }

        /// <summary>
        /// 根据类型获取物品
        /// </summary>
        /// <param name="type">物品类型</param>
        /// <returns>物品列表</returns>
        public static List<Item> GetItemsByType(string type)
        {
            List<Item> result = new List<Item>();
            foreach (var item in items.Values)
            {
                if (item.Type == type)
                {
                    result.Add(item);
                }
            }
            return result;
        }

        /// <summary>
        /// 根据等级获取物品
        /// </summary>
        /// <param name="level">物品等级</param>
        /// <returns>物品列表</returns>
        public static List<Item> GetItemsByLevel(string level)
        {
            List<Item> result = new List<Item>();
            foreach (var item in items.Values)
            {
                if (item.Level == level)
                {
                    result.Add(item);
                }
            }
            return result;
        }
    }
}
