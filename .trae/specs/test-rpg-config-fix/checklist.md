# XCell RPG 配置读取问题分析 - 验证清单

- [x] 配置文件读取：`ProjectSettings.toml` 能被正确读取和解析
- [x] 生成器配置：Cocos 和 Unity 生成器配置能被正确解析
- [x] 配置转换：生成器配置能正确从 `ProjectConfig` 转换为 `GeneratorConfig`
- [x] 配置传递：生成器配置能正确传递给代码生成模块
- [x] Cocos 产物：Cocos 产物能在指定的输出目录生成
- [x] Unity 产物：Unity 产物能在指定的输出目录生成
- [x] 脚本验证：`test-rpg.mjs` 脚本能成功执行并验证生成的产物
- [x] 错误处理：修复后不应引入新的错误或问题
- [x] 兼容性：修复应保持向后兼容性
- [x] 文档：必要的文档更新