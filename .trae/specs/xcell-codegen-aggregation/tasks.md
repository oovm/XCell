# XCell 代码生成能力聚合 - 实现计划（分解和优先级排序）

## [ ] 任务 1：修改 xcell 可执行文件，集成 xcell-generator 模块
- **优先级**：P0
- **依赖**：None
- **描述**：
  - 修改 `backends/xcell/src/main.rs` 文件，移除硬编码的代码生成逻辑
  - 集成 `xcell-generator` 模块，使用其生成器进行代码生成
  - 确保生成器能够正确处理工作区配置
- **验收标准**：AC-1
- **测试需求**：
  - `programmatic` TR-1.1：运行 `xcell` 命令时，应调用 `xcell-generator` 模块
  - `programmatic` TR-1.2：代码生成过程应无错误
- **备注**：需要在 Cargo.toml 中添加 xcell-generator 作为依赖

## [ ] 任务 2：实现基于模板的代码生成机制
- **优先级**：P0
- **依赖**：任务 1
- **描述**：
  - 确保 `xcell-generator` 使用 `templates` 目录中的模板文件
  - 实现模板加载和渲染逻辑
  - 验证模板系统能够正确生成代码
- **验收标准**：AC-2
- **测试需求**：
  - `programmatic` TR-2.1：代码生成应使用模板文件
  - `programmatic` TR-2.2：生成的代码应符合模板定义
- **备注**：检查现有模板文件是否完整

## [ ] 任务 3：修复 WorkspaceManager 的代码生成方法
- **优先级**：P1
- **依赖**：任务 1
- **描述**：
  - 修改 `WorkspaceManager` 的 `write_unity()` 和 `write_cocos()` 方法
  - 使其调用 `xcell-generator` 模块进行代码生成
  - 确保代码生成逻辑与 `xcell` 可执行文件一致
- **验收标准**：AC-1, AC-3, AC-4
- **测试需求**：
  - `programmatic` TR-3.1：`write_unity()` 方法应调用 `xcell-generator`
  - `programmatic` TR-3.2：`write_cocos()` 方法应调用 `xcell-generator`
- **备注**：确保方法签名和返回值类型正确

## [ ] 任务 4：确保 Cocos 平台代码生成正常工作
- **优先级**：P1
- **依赖**：任务 2
- **描述**：
  - 验证 `xcell-generator` 对 Cocos 平台的代码生成
  - 确保生成的 TypeScript 文件正确
  - 测试生成的代码是否可以正常编译
- **验收标准**：AC-3
- **测试需求**：
  - `programmatic` TR-4.1：运行 `test-rpg.mjs` 后，Cocos 目录应生成正确的 TypeScript 文件
  - `programmatic` TR-4.2：生成的 TypeScript 文件应包含正确的数据表定义
- **备注**：检查 Cocos 代码生成器的实现

## [ ] 任务 5：确保 Unity 平台代码生成正常工作
- **优先级**：P1
- **依赖**：任务 2
- **描述**：
  - 验证 `xcell-generator` 对 Unity 平台的代码生成
  - 确保生成的 C# 文件正确
  - 测试生成的代码是否可以正常编译
- **验收标准**：AC-4
- **测试需求**：
  - `programmatic` TR-5.1：运行 `test-rpg.mjs` 后，Unity 目录应生成正确的 C# 文件
  - `programmatic` TR-5.2：生成的 C# 文件应包含正确的数据表定义
- **备注**：检查 Unity 代码生成器的实现

## [ ] 任务 6：修复占位符文件问题
- **优先级**：P0
- **依赖**：任务 3, 任务 4, 任务 5
- **描述**：
  - 找出为什么会生成 `Placeholder.ts` 和 `Placeholder.cs` 文件
  - 修复代码生成逻辑，确保生成实际的数据表代码
  - 验证生成的文件是否正确
- **验收标准**：AC-5
- **测试需求**：
  - `programmatic` TR-6.1：运行 `test-rpg.mjs` 后，不应生成 `Placeholder.ts` 和 `Placeholder.cs` 文件
  - `programmatic` TR-6.2：应生成与数据表对应的代码文件
- **备注**：检查 `ProjectSettings.toml` 配置文件

## [ ] 任务 7：测试整体代码生成流程
- **优先级**：P1
- **依赖**：任务 4, 任务 5, 任务 6
- **描述**：
  - 运行 `test-rpg.mjs` 脚本测试整体代码生成流程
  - 验证所有平台的代码生成是否正常
  - 检查生成的代码是否正确
- **验收标准**：AC-3, AC-4, AC-5
- **测试需求**：
  - `programmatic` TR-7.1：`test-rpg.mjs` 脚本应成功执行
  - `programmatic` TR-7.2：生成的代码文件应正确且完整
- **备注**：确保测试环境配置正确