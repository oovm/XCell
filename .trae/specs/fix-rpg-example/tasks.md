# 修复 RPG 示例项目 - 实现计划

## [x] Task 1: 修复 ProjectSettings.toml 配置
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 修改 include 配置，使其支持 CSV 文件
  - 确保 Unity 生成功能启用
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-1.1: include 配置包含 CSV 文件格式
  - `programmatic` TR-1.2: Unity 生成启用
- **Notes**: 参考 rpg-typed 示例的配置（虽然它是空的）

## [x] Task 2: 修复 MonsterType.csv 文件格式
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - 添加类型定义行（第二行）
  - 确保格式与 rpg-typed 示例一致
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-2.1: 文件包含类型定义行
  - `programmatic` TR-2.2: 类型定义正确
- **Notes**: 使用 rpg-typed 示例的 MonsterType.csv 作为参考

## [x] Task 3: 检查并修复其他 CSV 文件
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - 检查 Item.csv, Monsters.csv, PlayerLevels.csv, Skills.csv
  - 确保它们有正确的格式
  - 如有需要，添加类型定义行
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-3.1: 所有文件格式检查通过
  - `programmatic` TR-3.2: 所有需要类型定义行的文件都有
- **Notes**: 检查每个文件是否需要类型定义行

## [/] Task 4: 测试 xcell 工具运行
- **Priority**: P0
- **Depends On**: Task 1, Task 2, Task 3
- **Description**:
  - 在 RPG 示例目录中运行 xcell
  - 确保无错误发生
- **Acceptance Criteria Addressed**: AC-3
- **Test Requirements**:
  - `programmatic` TR-4.1: xcell 成功运行
  - `programmatic` TR-4.2: 无错误信息输出
- **Notes**: 注意查看是否有警告或错误

## [ ] Task 5: 验证 Unity 代码生成
- **Priority**: P0
- **Depends On**: Task 4
- **Description**:
  - 检查 Unity 相关的代码和数据文件是否生成
  - 验证文件生成在正确位置
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `programmatic` TR-5.1: Unity 脚本文件生成
  - `programmatic` TR-5.2: Unity 数据文件生成
- **Notes**: 检查 ProjectSettings.toml 中配置的输出路径

## [ ] Task 6: 更新 test-rpg.mjs 脚本
- **Priority**: P1
- **Depends On**: Task 5
- **Description**:
  - 更新测试脚本，确保它正确测试 RPG 示例
  - 添加生成文件的验证
- **Acceptance Criteria Addressed**: AC-5
- **Test Requirements**:
  - `programmatic` TR-6.1: 测试脚本成功运行
  - `programmatic` TR-6.2: 测试脚本验证生成结果
- **Notes**: 确保测试脚本检查生成的文件

## [ ] Task 7: 运行完整测试
- **Priority**: P1
- **Depends On**: Task 6
- **Description**:
  - 运行 test-rpg.mjs 脚本
  - 验证所有功能正常工作
- **Acceptance Criteria Addressed**: AC-5
- **Test Requirements**:
  - `programmatic` TR-7.1: test-rpg.mjs 成功运行
  - `human-judgement` TR-7.2: 输出日志清晰
- **Notes**: 这是最终验证
