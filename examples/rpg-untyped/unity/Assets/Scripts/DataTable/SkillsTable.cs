using System;
using System.Collections.Generic;
using System.IO;
using UnityEngine;

namespace DataTable
{
    /// <summary>
    /// 技能表数据结构
    /// </summary>
    public class Skill
    {
        /// <summary>
        /// 技能ID
        /// </summary>
        public string Id { get; set; }
        /// <summary>
        /// 技能名称
        /// </summary>
        public string Name { get; set; }
        /// <summary>
        /// 技能描述
        /// </summary>
        public string Description { get; set; }
        /// <summary>
        /// 技能伤害
        /// </summary>
        public string Damage { get; set; }
        /// <summary>
        /// 技能冷却时间
        /// </summary>
        public string Cooldown { get; set; }
        /// <summary>
        /// 技能消耗法力值
        /// </summary>
        public string ManaCost { get; set; }
        /// <summary>
        /// 技能等级要求
        /// </summary>
        public string LevelRequirement { get; set; }
    }

    /// <summary>
    /// 技能表加载器
    /// </summary>
    public static class SkillsTable
    {
        private static Dictionary<string, Skill> skills = new Dictionary<string, Skill>();

        /// <summary>
        /// 加载技能表数据
        /// </summary>
        public static void Load()
        {
            // 这里可以从二进制文件或JSON文件加载数据
            // 暂时使用硬编码数据作为示例
            LoadSampleData();
            
            // 存储到 DataTableManager
            DataTableManager.Instance.SetTable("Skills", skills);
        }

        /// <summary>
        /// 加载示例数据
        /// </summary>
        private static void LoadSampleData()
        {
            skills.Clear();
            
            // 示例数据
            skills.Add("2", new Skill
            {
                Id = "2",
                Name = "强力攻击",
                Description = "更强大的攻击，造成更多伤害",
                Damage = "20",
                Cooldown = "2",
                ManaCost = "10",
                LevelRequirement = "2"
            });
            
            skills.Add("3", new Skill
            {
                Id = "3",
                Name = "火球术",
                Description = "火系法术，造成魔法伤害",
                Damage = "30",
                Cooldown = "3",
                ManaCost = "20",
                LevelRequirement = "3"
            });
            
            skills.Add("4", new Skill
            {
                Id = "4",
                Name = "治疗术",
                Description = "恢复生命值的治疗法术",
                Damage = "0",
                Cooldown = "4",
                ManaCost = "30",
                LevelRequirement = "3"
            });
        }

        /// <summary>
        /// 根据ID获取技能
        /// </summary>
        /// <param name="id">技能ID</param>
        /// <returns>技能实例</returns>
        public static Skill GetSkillById(string id)
        {
            if (skills.TryGetValue(id, out var skill))
            {
                return skill;
            }
            return null;
        }

        /// <summary>
        /// 获取所有技能
        /// </summary>
        /// <returns>技能列表</returns>
        public static List<Skill> GetAllSkills()
        {
            return new List<Skill>(skills.Values);
        }

        /// <summary>
        /// 根据等级要求获取技能
        /// </summary>
        /// <param name="level">玩家等级</param>
        /// <returns>技能列表</returns>
        public static List<Skill> GetSkillsByLevel(string level)
        {
            List<Skill> result = new List<Skill>();
            int playerLevel = int.Parse(level);
            foreach (var skill in skills.Values)
            {
                if (int.Parse(skill.LevelRequirement) <= playerLevel)
                {
                    result.Add(skill);
                }
            }
            return result;
        }
    }
}
