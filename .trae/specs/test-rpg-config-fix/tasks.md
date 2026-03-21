# XCell RPG 配置读取问题分析 - 实现计划

## [ ] Task 1: 分析配置文件解析问题
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 检查 `ProjectSettings.toml` 配置文件格式是否正确
  - 分析 `ProjectConfig::new` 方法的实现，确认配置文件读取逻辑
  - 分析 `CocosCodegen` 和 `UnityCodegen` 的反序列化实现
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-1.1: 验证配置文件能被正确读取
  - `programmatic` TR-1.2: 验证生成器配置能被正确解析
- **Notes**: 重点检查配置文件中的 `[[generators]]` 部分，确认是否与反序列化结构匹配

## [ ] Task 2: 分析生成器配置传递问题
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 分析 `GeneratorConfig::from_project_config` 方法的实现
  - 确认生成器配置是否正确从 `ProjectConfig` 转换为 `GeneratorConfig`
  - 检查生成器配置是否正确传递给代码生成模块
- **Acceptance Criteria Addressed**: AC-1, AC-2, AC-3
- **Test Requirements**:
  - `programmatic` TR-2.1: 验证生成器配置能正确转换
  - `programmatic` TR-2.2: 验证生成器配置能正确传递
- **Notes**: 重点检查 `from_project_config` 方法中对 Cocos 和 Unity 生成器的处理

## [ ] Task 3: 分析代码生成模块问题
- **Priority**: P0
- **Depends On**: Task 2
- **Description**: 
  - 分析 `Generator::generate` 方法的实现
  - 确认代码生成模块是否正确处理生成请求
  - 检查输出目录是否正确创建
- **Acceptance Criteria Addressed**: AC-2, AC-3
- **Test Requirements**:
  - `programmatic` TR-3.1: 验证代码生成模块能正确处理生成请求
  - `programmatic` TR-3.2: 验证输出目录能正确创建
- **Notes**: 重点检查生成器的 `generate` 方法，确认是否调用了正确的生成逻辑

## [ ] Task 4: 修复配置文件读取问题
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 根据分析结果，修复配置文件读取问题
  - 确保 `ProjectSettings.toml` 能被正确解析
  - 确保生成器配置能被正确读取
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-4.1: 验证修复后配置文件能被正确读取
  - `programmatic` TR-4.2: 验证修复后生成器配置能被正确解析
- **Notes**: 可能需要修改反序列化逻辑或配置文件格式

## [ ] Task 5: 修复生成器配置传递问题
- **Priority**: P0
- **Depends On**: Task 2, Task 4
- **Description**: 
  - 根据分析结果，修复生成器配置传递问题
  - 确保生成器配置能正确从 `ProjectConfig` 转换为 `GeneratorConfig`
  - 确保生成器配置能正确传递给代码生成模块
- **Acceptance Criteria Addressed**: AC-1, AC-2, AC-3
- **Test Requirements**:
  - `programmatic` TR-5.1: 验证修复后生成器配置能正确转换
  - `programmatic` TR-5.2: 验证修复后生成器配置能正确传递
- **Notes**: 可能需要修改 `from_project_config` 方法的实现

## [ ] Task 6: 修复代码生成模块问题
- **Priority**: P0
- **Depends On**: Task 3, Task 5
- **Description**: 
  - 根据分析结果，修复代码生成模块问题
  - 确保代码生成模块能正确处理生成请求
  - 确保输出目录能正确创建
- **Acceptance Criteria Addressed**: AC-2, AC-3
- **Test Requirements**:
  - `programmatic` TR-6.1: 验证修复后代码生成模块能正确处理生成请求
  - `programmatic` TR-6.2: 验证修复后输出目录能正确创建
- **Notes**: 可能需要修改 `Generator::generate` 方法的实现

## [ ] Task 7: 验证修复结果
- **Priority**: P0
- **Depends On**: Task 4, Task 5, Task 6
- **Description**: 
  - 运行 `test-rpg.mjs` 脚本验证修复结果
  - 检查 Cocos 和 Unity 产物是否成功生成
  - 确认脚本能成功验证生成的产物
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `programmatic` TR-7.1: 验证 `test-rpg.mjs` 脚本能成功执行
  - `programmatic` TR-7.2: 验证 Cocos 产物能成功生成
  - `programmatic` TR-7.3: 验证 Unity 产物能成功生成
- **Notes**: 重点检查脚本的验证逻辑，确保能正确检测生成的产物