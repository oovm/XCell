# XCell 代码生成规范 - 实现计划

## [ ] 任务 1：检查代码生成模块，找出使用手工字符串拼接的地方
- **优先级**：P0
- **依赖**：None
- **描述**：
  - 检查 XCell 项目中所有代码生成模块
  - 找出使用手工字符串拼接的地方
  - 记录需要修改的文件和位置
- **验收标准**：AC-1
- **测试要求**：
  - `human-judgment` TR-1.1：检查所有代码生成模块，确认哪些使用了手工字符串拼接
- **备注**：重点检查 Unity 代码生成模块，因为之前的分析显示它使用了手工字符串拼接

## [ ] 任务 2：检查 dejavu-engine 的 Template 宏实现，确保支持必要特性
- **优先级**：P0
- **依赖**：任务 1
- **描述**：
  - 检查 dejavu-engine 的 Template 宏实现
  - 确认是否支持 XCell 代码生成所需的所有特性
  - 如有必要，在 dejavu-engine 中添加缺失的特性
- **验收标准**：AC-2
- **测试要求**：
  - `programmatic` TR-2.1：验证 Template 宏支持 XCell 代码生成所需的所有特性
- **备注**：需要确保 Template 宏支持结构体字段的访问、条件语句、循环等功能

## [ ] 任务 3：修改 Unity 代码生成模块使用 Template 宏
- **优先级**：P1
- **依赖**：任务 2
- **描述**：
  - 修改 Unity 代码生成模块，使用 `#[derive(Template)]` 宏
  - 为 Unity 代码生成创建相应的模板文件
  - 确保生成的代码与之前保持一致
- **验收标准**：AC-1, AC-3
- **测试要求**：
  - `human-judgment` TR-3.1：确认 Unity 代码生成模块使用了 Template 宏
  - `programmatic` TR-3.2：验证 Unity 代码生成功能正常
- **备注**：重点修改 `write_class` 和 `write_manager` 方法

## [ ] 任务 4：检查其他代码生成模块，确保使用 Template 宏
- **优先级**：P1
- **依赖**：任务 2
- **描述**：
  - 检查其他代码生成模块（如 TypeScript、XLua 等）
  - 确保它们都使用了 `#[derive(Template)]` 宏
  - 如有必要，进行修改
- **验收标准**：AC-1, AC-3
- **测试要求**：
  - `human-judgment` TR-4.1：确认所有代码生成模块都使用了 Template 宏
  - `programmatic` TR-4.2：验证所有代码生成功能正常
- **备注**：Cocos 模块已经使用了 Template 宏，需要确认其他模块

## [ ] 任务 5：运行测试脚本，验证代码生成功能正常
- **优先级**：P0
- **依赖**：任务 3, 任务 4
- **描述**：
  - 运行测试脚本 `generate-table.mjs`
  - 验证所有代码生成功能正常
  - 检查生成的代码是否正确
- **验收标准**：AC-3
- **测试要求**：
  - `programmatic` TR-5.1：运行测试脚本，确认没有错误
  - `human-judgment` TR-5.2：检查生成的代码是否正确
- **备注**：测试脚本位于 `e:\灵之镜有限公司\XCell\scripts\generate-table.mjs`