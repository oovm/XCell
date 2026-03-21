# XCell 代码生成能力聚合 - 验证检查清单

- [ ] 检查 `xcell` 可执行文件是否使用 `xcell-generator` 模块进行代码生成
- [ ] 检查 `xcell-generator` 是否使用 `templates` 目录中的模板文件
- [ ] 检查 `WorkspaceManager` 的 `write_unity()` 和 `write_cocos()` 方法是否调用 `xcell-generator`
- [ ] 检查 Cocos 平台是否生成正确的 TypeScript 文件
- [ ] 检查 Unity 平台是否生成正确的 C# 文件
- [ ] 检查是否不再生成 `Placeholder.ts` 和 `Placeholder.cs` 文件
- [ ] 检查 `test-rpg.mjs` 脚本是否成功执行
- [ ] 检查生成的代码文件是否包含正确的数据表定义
- [ ] 检查代码生成过程是否无错误
- [ ] 检查生成的代码是否可以正常编译