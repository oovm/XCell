# 数据表生成修复 - 实现计划

## [x] 任务 1：分析生成的表与原始表不匹配的原因
- **优先级**：P0
- **依赖**：None
- **描述**：
  - 分析 SLG 和 Galgame 游戏的原始 CSV 表结构
  - 分析生成的 Cocos 和 Unity 文件结构
  - 找出不匹配的具体原因
- **验收标准**：AC-1, AC-2, AC-3
- **测试要求**：
  - `programmatic` TR-1.1: 比较原始表和生成表的字段数量和名称
  - `programmatic` TR-1.2: 检查生成表是否包含原始表的所有字段
- **备注**：重点关注 SLG 的 Units.csv, Buildings.csv, Technologies.csv 和 Galgame 的 Characters.csv, Scenes.csv, Dialogues.csv

## [x] 任务 2：修复 SLG 游戏的数据表生成
- **优先级**：P0
- **依赖**：任务 1
- **描述**：
  - 确保生成的 SLG 游戏 Cocos TypeScript 文件包含原始表的所有字段
  - 确保生成的 SLG 游戏 Unity C# 文件包含原始表的所有字段
  - 确保生成的 SLG 游戏数据表管理器包含所有表的加载和管理方法
- **验收标准**：AC-1, AC-3, AC-4
- **测试要求**：
  - `programmatic` TR-2.1: 验证 SLG 生成文件包含原始表的所有字段
  - `programmatic` TR-2.2: 验证 SLG 数据表管理器包含所有表的加载方法
  - `human-judgment` TR-2.3: 检查生成代码的质量和注释
- **备注**：重点修复 Units, Buildings, Technologies 表的生成

## [x] 任务 3：修复 Galgame 游戏的数据表生成
- **优先级**：P0
- **依赖**：任务 1
- **描述**：
  - 确保生成的 Galgame 游戏 Cocos TypeScript 文件包含原始表的所有字段
  - 确保生成的 Galgame 游戏 Unity C# 文件包含原始表的所有字段
  - 确保生成的 Galgame 游戏数据表管理器包含所有表的加载和管理方法
- **验收标准**：AC-2, AC-3, AC-4
- **测试要求**：
  - `programmatic` TR-3.1: 验证 Galgame 生成文件包含原始表的所有字段
  - `programmatic` TR-3.2: 验证 Galgame 数据表管理器包含所有表的加载方法
  - `human-judgment` TR-3.3: 检查生成代码的质量和注释
- **备注**：重点修复 Characters, Scenes, Dialogues 表的生成

## [x] 任务 4：测试数据表生成过程
- **优先级**：P1
- **依赖**：任务 2, 任务 3
- **描述**：
  - 运行 XCell 生成工具，测试修复后的生成过程
  - 验证生成的文件是否正确反映原始表结构
  - 检查生成过程是否稳定无错误
- **验收标准**：AC-1, AC-2, AC-3, AC-4
- **测试要求**：
  - `programmatic` TR-4.1: 运行生成工具，检查是否有错误
  - `programmatic` TR-4.2: 验证生成的文件结构与原始表一致
  - `human-judgment` TR-4.3: 检查生成代码的质量
- **备注**：确保生成过程能够正常完成，无错误

## [x] 任务 5：验证生成的数据表功能
- **优先级**：P1
- **依赖**：任务 4
- **描述**：
  - 验证生成的数据表文件是否可以正常加载
  - 验证数据表管理器是否可以正确管理所有表数据
  - 确保生成的数据表可以在游戏中正常使用
- **验收标准**：AC-3, AC-4
- **测试要求**：
  - `programmatic` TR-5.1: 验证数据表文件可以正常加载
  - `programmatic` TR-5.2: 验证数据表管理器的方法是否正常工作
  - `human-judgment` TR-5.3: 检查生成代码的可维护性
- **备注**：确保生成的数据表在游戏开发中可以正常使用