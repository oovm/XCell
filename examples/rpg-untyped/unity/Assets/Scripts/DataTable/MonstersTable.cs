using System;
using System.Collections.Generic;
using System.IO;
using UnityEngine;

namespace DataTable
{
    /// <summary>
    /// 怪物表数据结构
    /// </summary>
    public class Monster
    {
        /// <summary>
        /// 怪物ID
        /// </summary>
        public string Id { get; set; }
        /// <summary>
        /// 怪物名称
        /// </summary>
        public string Name { get; set; }
        /// <summary>
        /// 怪物等级
        /// </summary>
        public string Level { get; set; }
        /// <summary>
        /// 怪物生命值
        /// </summary>
        public string Health { get; set; }
        /// <summary>
        /// 怪物伤害
        /// </summary>
        public string Damage { get; set; }
        /// <summary>
        /// 怪物防御力
        /// </summary>
        public string Defense { get; set; }
        /// <summary>
        /// 怪物类型
        /// </summary>
        public string Type { get; set; }
        /// <summary>
        /// 掉落物品
        /// </summary>
        public string DropItems { get; set; }
        /// <summary>
        /// 怪物技能
        /// </summary>
        public string Skills { get; set; }
    }

    /// <summary>
    /// 怪物表加载器
    /// </summary>
    public static class MonstersTable
    {
        private static Dictionary<string, Monster> monsters = new Dictionary<string, Monster>();

        /// <summary>
        /// 加载怪物表数据
        /// </summary>
        public static void Load()
        {
            // 这里可以从二进制文件或JSON文件加载数据
            // 暂时使用硬编码数据作为示例
            LoadSampleData();
            
            // 存储到 DataTableManager
            DataTableManager.Instance.SetTable("Monsters", monsters);
        }

        /// <summary>
        /// 加载示例数据
        /// </summary>
        private static void LoadSampleData()
        {
            monsters.Clear();
            
            // 示例数据
            monsters.Add("2", new Monster
            {
                Id = "2",
                Name = "兽人",
                Level = "2",
                Health = "80",
                Damage = "8",
                Defense = "4",
                Type = "normal",
                DropItems = "3",
                Skills = "4"
            });
            
            monsters.Add("3", new Monster
            {
                Id = "3",
                Name = "骷髅",
                Level = "2",
                Health = "60",
                Damage = "6",
                Defense = "3",
                Type = "undead",
                DropItems = "2",
                Skills = "5"
            });
            
            monsters.Add("4", new Monster
            {
                Id = "4",
                Name = "巨型蜘蛛",
                Level = "3",
                Health = "100",
                Damage = "10",
                Defense = "5",
                Type = "beast",
                DropItems = "4",
                Skills = "6"
            });
        }

        /// <summary>
        /// 根据ID获取怪物
        /// </summary>
        /// <param name="id">怪物ID</param>
        /// <returns>怪物实例</returns>
        public static Monster GetMonsterById(string id)
        {
            if (monsters.TryGetValue(id, out var monster))
            {
                return monster;
            }
            return null;
        }

        /// <summary>
        /// 获取所有怪物
        /// </summary>
        /// <returns>怪物列表</returns>
        public static List<Monster> GetAllMonsters()
        {
            return new List<Monster>(monsters.Values);
        }

        /// <summary>
        /// 根据类型获取怪物
        /// </summary>
        /// <param name="type">怪物类型</param>
        /// <returns>怪物列表</returns>
        public static List<Monster> GetMonstersByType(string type)
        {
            List<Monster> result = new List<Monster>();
            foreach (var monster in monsters.Values)
            {
                if (monster.Type == type)
                {
                    result.Add(monster);
                }
            }
            return result;
        }

        /// <summary>
        /// 根据等级获取怪物
        /// </summary>
        /// <param name="level">怪物等级</param>
        /// <returns>怪物列表</returns>
        public static List<Monster> GetMonstersByLevel(string level)
        {
            List<Monster> result = new List<Monster>();
            foreach (var monster in monsters.Values)
            {
                if (monster.Level == level)
                {
                    result.Add(monster);
                }
            }
            return result;
        }
    }
}
