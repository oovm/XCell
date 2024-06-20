# xcell-generator

代码生成器模块，为 XCell 提供各种目标平台的代码生成功能。

## 功能特性

- 支持多种目标平台的代码生成
- 基于模板的代码生成系统
- 与 XCell 核心类型系统集成
- 支持 Dejavu 模板的编译期静态生成

## 支持的目标平台

- Cocos
- JSON
- SQL
- TypeScript
- Unity
- XLua
- Dejavu (静态生成)

## 使用方法

### 基本使用

```rust
use xcell_generator::codegen::*;

// 生成代码示例
let generator = UnityGenerator::new();
generator.generate(&table_data, &output_path);
```

### Dejavu 静态生成

```rust
use xcell_generator::codegen::*;
use std::collections::HashMap;

// 创建代码生成上下文
let mut options = HashMap::new();
options.insert("template_path".to_string(), "path/to/template.dj".to_string());
options.insert("package_name".to_string(), "my_package".to_string());

let context = CodegenContext {
    output_dir: "path/to/output".to_string(),
    options,
    global_options: HashMap::new(),
};

// 创建 Dejavu 代码生成器
let generator = DejavuCodegen::new();

// 生成代码
generator.generate(&context).expect("Failed to generate code");
```

## Dejavu 模板语法

Dejavu 模板使用 `<$` 和 `$>` 作为分隔符：

```
Hello, <$ name $>!
You are <$ age $> years old.
```

## 配置选项

- `template_path`: 模板文件路径（必需）
- `output_dir`: 输出目录（必需）
- `package_name`: 包名（可选）
- `optimize`: 是否优化生成的代码（可选，默认 true）
- `debug`: 是否生成调试信息（可选，默认 false）
- `forced_class_name`: 强制指定生成的类名（可选）
- `imports`: 额外的导入语句，使用分号分隔（可选）

## 依赖

- xcell-types: XCell 核心类型系统
- dejavu-types: 代码生成模板系统
- serde: 序列化/反序列化支持
- tracing: 日志支持
