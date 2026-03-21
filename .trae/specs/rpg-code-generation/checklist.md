# XCell RPG 代码生成问题分析 - 验证检查清单

- [ ] 检查 ProjectSettings.toml 文件是否存在且格式正确
- [ ] 检查 ProjectSettings.toml 文件中是否包含 cocos 和 unity 生成器配置
- [ ] 检查生成器配置是否包含必要的启用选项
- [ ] 检查生成器配置中的输出路径是否正确设置
- [ ] 检查 xcell 可执行文件是否存在且可执行
- [ ] 检查 xcell generate 命令是否能正确读取 ProjectSettings.toml 文件
- [ ] 检查 xcell generate 命令是否能正确解析生成器配置
- [ ] 检查 xcell generate 命令是否能成功执行
- [ ] 检查 cocos 目录是否生成了相应的代码产物
- [ ] 检查 unity 目录是否生成了相应的代码产物
- [ ] 检查生成的代码产物是否符合预期
- [ ] 检查 test-rpg.mjs 脚本是否能成功执行
- [ ] 检查 test-rpg.mjs 脚本是否能正确验证产物存在
- [ ] 检查多次执行 xcell generate 命令是否都能成功
- [ ] 检查修改配置文件后是否能正确应用新的配置
- [ ] 检查修复后的功能是否保持向后兼容性
- [ ] 检查修复后的功能是否在 Windows 平台正常运行