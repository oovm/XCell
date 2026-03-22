# XCell 硬编码和作弊行为检测报告

## 检测时间
2026-03-22

## 检测结果

### 1. 硬编码的文件路径

#### 脚本文件中的硬编码路径
- **scripts/generate-table.mjs**:
  - `path.resolve(projectRoot, 'target', 'debug', 'xcell.exe')` - 硬编码的可执行文件路径
  - `path.resolve(rpgDir, 'Assets', 'Scripts', 'DataTable', 'Generated')` - 硬编码的Unity生成目录
  - `path.resolve(rpgDir, '..', 'assets', 'scripts', 'dataTable', 'generated')` - 硬编码的Cocos生成目录

- **scripts/build-wasi.mjs**:
  - `join(ROOT_DIR, "target", "wasm32-wasip1", "release", "xcell_wasi.wasm")` - 硬编码的WASM文件路径
  - `join(FRONTEND_LIB_DIR, "xcell_wasi.js")` - 硬编码的JS文件路径

#### 配置文件中的硬编码路径
- **backends/xcell-config/test_config.rs**:
  - `Path::new("e:\\灵之镜有限公司\\XCell\\examples\\rpg-untyped\\ProjectSettings.toml")` - 硬编码的Windows路径
  - `Path::new("e:\\灵之镜有限公司\\XCell\\backends\\xcell-config\\ProjectConfig.toml")` - 硬编码的Windows路径

### 2. 硬编码的字符串常量

#### 模板路径
- **backends/xcell-generator/src/codegen/cocos/mod.rs**:
  - `#[template(path = "CocosEnumerate.ts", ext = "dejavu", escape = "none")]`
  - `#[template(path = "CocosClass.ts", ext = "dejavu", escape = "none")]`
  - `#[template(path = "CocosDataTableManager.ts", ext = "dejavu", escape = "none")]`

#### 类型映射
- **backends/xcell-generator/src/codegen/cocos/mod.rs**:
  - `"i32" | "i64" | "u32" | "u64" | "f32" | "f64" => "number".to_string(),`
  - `"text" | "string" => { ... }`
  - `"any" => "string".to_string(),`
  - `_ => "string".to_string(),`

#### 字段名检查
- **backends/xcell-generator/src/codegen/cocos/mod.rs**:
  - `let has_type_field = fields.iter().any(|f| f.name == "type");`
  - `let has_level_field = fields.iter().any(|f| f.name == "level") || fields.iter().any(|f| f.name == "level_requirement");`
  - `ctx.set_var("is_monster".to_string(), (class_name == "Monster").to_dejavu_value());`
  - `ctx.set_var("is_skill".to_string(), (class_name == "Skill").to_dejavu_value());`

#### 表头索引
- **backends/xcell-generator/src/codegen/cocos/mod.rs**:
  - `let id_index = headers.iter().position(|h| h == "id").unwrap_or(0);`
  - `let name_index = headers.iter().position(|h| h == "name").unwrap_or(1);`
  - `let desc_index = headers.iter().position(|h| h == "description" || h == "desc").unwrap_or_else(|| { ... });`

### 3. 潜在的作弊行为

#### 调试模式
- **frontends/xcell-desktop/src/main.rs**:
  - `#[cfg(debug_assertions)]` - 条件编译的调试代码

#### 执行命令
- **scripts/generate-table.mjs**:
  - `execSync(`"${xcellPath}"`, { cwd: rpgDir, stdio: 'inherit' });` - 执行外部命令

#### 权限配置
- **frontends/xcell-desktop/gen/schemas/desktop-schema.json**:
  - 包含多个权限配置，如 `shell:allow-execute`、`shell:allow-spawn` 等

## 风险评估

### 高风险
- **硬编码的路径** - 可能导致在不同环境下路径错误
- **硬编码的类型映射** - 可能导致类型转换错误

### 中风险
- **硬编码的字段名检查** - 可能导致字段识别错误
- **执行外部命令** - 可能存在安全风险

### 低风险
- **调试模式代码** - 正常的开发功能
- **权限配置** - 必要的功能配置

## 修复建议

### 1. 硬编码路径修复
- 将硬编码路径移至配置文件
- 使用环境变量或配置对象管理路径
- 增加路径配置的灵活性

### 2. 硬编码字符串修复
- 将类型映射和字段名检查移至配置文件
- 使用配置驱动的方式管理类型映射
- 增加字段名检查的可配置性

### 3. 安全风险修复
- 对外部命令执行增加安全检查
- 确保权限配置符合最小权限原则
- 增加输入验证

## 结论

XCell 项目中存在一些硬编码问题，但没有发现明显的作弊行为或后门。建议按照修复建议进行改进，提高代码的可维护性和安全性。
