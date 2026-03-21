# XCell 代码生成问题修复 - 实现计划

## [ ] 任务 1: 分析占位符文件生成原因
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 分析为什么当前会生成占位符文件而不是实际代码
  - 检查 WorkspaceManager 和代码生成流程
  - 确定问题的根本原因
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-1.1: 运行 test-rpg.mjs 脚本，确认生成的是占位符文件
  - `human-judgment` TR-1.2: 分析代码生成流程，找出占位符文件生成的原因
- **Notes**: 重点检查 WorkspaceManager 的 first_walk 方法和代码生成调用

## [ ] 任务 2: 聚合代码生成能力到 xcell-generator 模块
- **Priority**: P0
- **Depends On**: 任务 1
- **Description**: 
  - 将所有代码生成能力从其他模块迁移到 xcell-generator 模块
  - 确保 xcell-generator 模块能够独立处理所有代码生成任务
  - 移除其他模块中的代码生成逻辑
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `human-judgment` TR-2.1: 检查 xcell-generator 模块的代码结构，确认所有代码生成能力都已聚合
  - `programmatic` TR-2.2: 编译项目，确保没有编译错误
- **Notes**: 注意避免循环依赖问题

## [ ] 任务 3: 实现模板系统集成
- **Priority**: P0
- **Depends On**: 任务 2
- **Description**: 
  - 确保 xcell-generator 模块使用 templates 目录中的模板文件
  - 实现 derive Template 调用模板系统
  - 测试模板系统的正确性
- **Acceptance Criteria Addressed**: AC-3
- **Test Requirements**:
  - `human-judgment` TR-3.1: 检查代码中是否使用了 derive Template
  - `programmatic` TR-3.2: 测试模板系统是否能正确生成代码
- **Notes**: 确保模板文件路径配置正确

## [ ] 任务 4: 实现 Cocos 平台代码生成
- **Priority**: P1
- **Depends On**: 任务 3
- **Description**: 
  - 实现 Cocos 平台的代码生成逻辑
  - 确保生成的 TypeScript 代码符合 Cocos 平台的规范
  - 测试 Cocos 平台代码生成的正确性
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `programmatic` TR-4.1: 运行 test-rpg.mjs 脚本，检查 Cocos 目录中是否生成了正确的 TypeScript 代码文件
  - `human-judgment` TR-4.2: 检查生成的 TypeScript 代码是否符合 Cocos 平台的规范
- **Notes**: 重点实现 CocosCodegen 的 generate 方法

## [ ] 任务 5: 实现 Unity 平台代码生成
- **Priority**: P1
- **Depends On**: 任务 3
- **Description**: 
  - 实现 Unity 平台的代码生成逻辑
  - 确保生成的 C# 代码符合 Unity 平台的规范
  - 测试 Unity 平台代码生成的正确性
- **Acceptance Criteria Addressed**: AC-5
- **Test Requirements**:
  - `programmatic` TR-5.1: 运行 test-rpg.mjs 脚本，检查 Unity 目录中是否生成了正确的 C# 代码文件
  - `human-judgment` TR-5.2: 检查生成的 C# 代码是否符合 Unity 平台的规范
- **Notes**: 重点实现 UnityCodegenWrapper 的 generate 方法

## [ ] 任务 6: 测试整体代码生成流程
- **Priority**: P1
- **Depends On**: 任务 4, 任务 5
- **Description**: 
  - 运行 test-rpg.mjs 脚本，测试整体代码生成流程
  - 检查生成的文件是否正确，没有占位符文件
  - 确保所有平台的代码生成都正常工作
- **Acceptance Criteria Addressed**: AC-1, AC-4, AC-5
- **Test Requirements**:
  - `programmatic` TR-6.1: 运行 test-rpg.mjs 脚本，确认生成的是实际代码文件而不是占位符文件
  - `programmatic` TR-6.2: 检查所有平台的生成文件是否正确
- **Notes**: 确保测试覆盖所有 RPG 示例目录