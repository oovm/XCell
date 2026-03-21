using System;
using System.Collections.Generic;
using System.IO;
using UnityEngine;

namespace DataTable
{
    /// <summary>
    /// 玩家等级表数据结构
    /// </summary>
    public class PlayerLevel
    {
        /// <summary>
        /// 等级ID
        /// </summary>
        public string Id { get; set; }
        /// <summary>
        /// 等级名称
        /// </summary>
        public string Name { get; set; }
        /// <summary>
        /// 所需经验值
        /// </summary>
        public string RequiredExp { get; set; }
        /// <summary>
        /// 生命值
        /// </summary>
        public string Health { get; set; }
        /// <summary>
        /// 伤害值
        /// </summary>
        public string Damage { get; set; }
        /// <summary>
        /// 防御力
        /// </summary>
        public string Defense { get; set; }
        /// <summary>
        /// 解锁技能
        /// </summary>
        public string UnlockSkills { get; set; }
    }

    /// <summary>
    /// 玩家等级表加载器
    /// </summary>
    public static class PlayerLevelsTable
    {
        private static Dictionary<string, PlayerLevel> playerLevels = new Dictionary<string, PlayerLevel>();

        /// <summary>
        /// 加载玩家等级表数据
        /// </summary>
        public static void Load()
        {
            // 这里可以从二进制文件或JSON文件加载数据
            // 暂时使用硬编码数据作为示例
            LoadSampleData();
            
            // 存储到 DataTableManager
            DataTableManager.Instance.SetTable("PlayerLevels", playerLevels);
        }

        /// <summary>
        /// 加载示例数据
        /// </summary>
        private static void LoadSampleData()
        {
            playerLevels.Clear();
            
            // 示例数据
            playerLevels.Add("2", new PlayerLevel
            {
                Id = "2",
                Name = "2级",
                RequiredExp = "1000",
                Health = "120",
                Damage = "15",
                Defense = "8",
                UnlockSkills = "1"
            });
            
            playerLevels.Add("3", new PlayerLevel
            {
                Id = "3",
                Name = "3级",
                RequiredExp = "3000",
                Health = "150",
                Damage = "20",
                Defense = "12",
                UnlockSkills = "1"
            });
            
            playerLevels.Add("4", new PlayerLevel
            {
                Id = "4",
                Name = "4级",
                RequiredExp = "6000",
                Health = "180",
                Damage = "25",
                Defense = "16",
                UnlockSkills = "1"
            });
        }

        /// <summary>
        /// 根据ID获取玩家等级
        /// </summary>
        /// <param name="id">等级ID</param>
        /// <returns>玩家等级实例</returns>
        public static PlayerLevel GetPlayerLevelById(string id)
        {
            if (playerLevels.TryGetValue(id, out var level))
            {
                return level;
            }
            return null;
        }

        /// <summary>
        /// 获取所有玩家等级
        /// </summary>
        /// <returns>玩家等级列表</returns>
        public static List<PlayerLevel> GetAllPlayerLevels()
        {
            return new List<PlayerLevel>(playerLevels.Values);
        }
    }
}
