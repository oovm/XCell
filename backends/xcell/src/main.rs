use clap::Parser;
use xcell::{SubArgs, TomlSubArgs, XCellArgs, logger, pause};
use xcell_analyzer::{WorkspaceManager, XResult, XError};
use xcell_config::project::Generator;
use std::path::PathBuf;
use oak_toml::{TomlValue, TomlTable, TomlArray, to_string, from_str};

#[tokio::main]
async fn main() -> XResult<()> {
    let args = XCellArgs::parse();
    logger(args.verbose, args.quiet);
    let filter = if args.filter.is_empty() { None } else { Some(args.filter.as_str()) };
    let result = match args.command {
        Some(SubArgs::Check) => {
            let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
            ws.first_walk(filter)?;
            Ok(())
        }
        Some(SubArgs::Clear) => {
            let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
            // 清空文件修改时间记录
            ws.file_modification_times.clear();
            tracing::info!("已清除工作空间缓存");
            Ok(())
        }
        Some(SubArgs::Toml { subcommand }) => match subcommand {
            TomlSubArgs::List { file } => {
                let content = std::fs::read_to_string(&file)?;
                let table: TomlValue = from_str(&content).map_err(|e| XError::runtime_error(format!("TOML parse error: {}", e)))?;
                if let Some(fields) = table.get("fields").and_then(|v| v.as_array()) {
                    println!("Fields in {}", file);
                    println!("{:-<50}", "");
                    for (i, field) in fields.iter().enumerate() {
                        let name = field.get("name").and_then(|v| v.as_str()).unwrap_or("");
                        let type_name = field.get("type").and_then(|v| v.as_str()).unwrap_or("");
                        let comment = field.get("comment").and_then(|v| v.as_str()).unwrap_or("");
                        let default = field.get("default").and_then(|v| v.as_str()).unwrap_or("");
                        println!("{}: {} ({}), comment: {}, default: {}", i + 1, name, type_name, comment, default);
                    }
                }
                else {
                    println!("No fields found in {}", file);
                }
                Ok(())
            }
            TomlSubArgs::Add { file, name, r#type, comment, default } => {
                let mut table: TomlValue = if std::path::Path::new(&file).exists() {
                    let content = std::fs::read_to_string(&file)?;
                    from_str(&content).map_err(|e| XError::runtime_error(format!("TOML parse error: {}", e)))?
                }
                else {
                    TomlValue::Table(TomlTable { dict: std::collections::HashMap::new() })
                };
                let fields = table.get_mut("fields").and_then(|v| v.as_array_mut());
                let fields = match fields {
                    Some(fields) => fields,
                    None => {
                        let new_array = TomlValue::Array(TomlArray { list: vec![] });
                        if let TomlValue::Table(ref mut table) = table {
                            table.dict.insert("fields".to_string(), new_array);
                        }
                        table.get_mut("fields").and_then(|v| v.as_array_mut()).unwrap()
                    }
                };
                let mut field = TomlValue::Table(TomlTable { dict: std::collections::HashMap::new() });
                if let TomlValue::Table(ref mut field_table) = field {
                    field_table.dict.insert("name".to_string(), TomlValue::String(name.clone()));
                    field_table.dict.insert("type".to_string(), TomlValue::String(r#type));
                    if let Some(comment) = comment {
                        field_table.dict.insert("comment".to_string(), TomlValue::String(comment));
                    }
                    if let Some(default) = default {
                        field_table.dict.insert("default".to_string(), TomlValue::String(default));
                    }
                }
                fields.push(field);
                let content = to_string(&table).map_err(|e| XError::runtime_error(format!("TOML serialize error: {}", e)))?;
                std::fs::write(&file, content)?;
                println!("Added field {} to {}", name, file);
                Ok(())
            }
            TomlSubArgs::Remove { file, name } => {
                let content = std::fs::read_to_string(&file)?;
                let mut table: TomlValue = from_str(&content).map_err(|e| XError::runtime_error(format!("TOML parse error: {}", e)))?;
                if let Some(fields) = table.get_mut("fields").and_then(|v| v.as_array_mut()) {
                    let initial_len = fields.len();
                    fields.retain(|field| field.get("name").and_then(|v| v.as_str()) != Some(&name));
                    if fields.len() < initial_len {
                        let content = to_string(&table).map_err(|e| XError::runtime_error(format!("TOML serialize error: {}", e)))?;
                        std::fs::write(&file, content)?;
                        println!("Removed field {} from {}", name, file);
                    }
                    else {
                        println!("Field {} not found in {}", name, file);
                    }
                }
                else {
                    println!("No fields found in {}", file);
                }
                Ok(())
            }
            TomlSubArgs::Update { file, name, r#type, comment, default } => {
                let content = std::fs::read_to_string(&file)?;
                let mut table: TomlValue = from_str(&content).map_err(|e| XError::runtime_error(format!("TOML parse error: {}", e)))?;
                if let Some(fields) = table.get_mut("fields").and_then(|v| v.as_array_mut()) {
                    let mut found = false;
                    for field in fields {
                        if let TomlValue::Table(ref mut field_table) = field {
                            if field_table.dict.get("name").and_then(|v| v.as_str()) == Some(&name) {
                                if let Some(r#type) = r#type {
                                    field_table.dict.insert("type".to_string(), TomlValue::String(r#type));
                                }
                                if let Some(comment) = comment {
                                    field_table.dict.insert("comment".to_string(), TomlValue::String(comment));
                                }
                                if let Some(default) = default {
                                    field_table.dict.insert("default".to_string(), TomlValue::String(default));
                                }
                                found = true;
                                break;
                            }
                        }
                    }
                    if found {
                        let content = to_string(&table).map_err(|e| XError::runtime_error(format!("TOML serialize error: {}", e)))?;
                        std::fs::write(&file, content)?;
                        println!("Updated field {} in {}", name, file);
                    }
                    else {
                        println!("Field {} not found in {}", name, file);
                    }
                }
                else {
                    println!("No fields found in {}", file);
                }
                Ok(())
            }
        },
        Some(SubArgs::Info) => {
            let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
            ws.first_walk(filter)?;
            println!("{}", ws.summary());
            Ok(())
        }
        Some(SubArgs::Init) => {
            let workspace = args.resolve_workspace()?;
            let config_path = workspace.join("ProjectConfig.toml");
            if config_path.exists() {
                tracing::warn!("工作空间已存在: {}", config_path.display());
                return Ok(());
            }
            let default_config = r#"[project]
include = ["**/*.xlsx", "**/*.xls", "**/*.csv", "**/*.tsv"]

[typing.enumerate]
integer = "i32"

[typing.language]
id = ["languageid"]
key = ["languagekey"]
value = ["languagevalue"]
group = ["languagegroup"]
"#;
            std::fs::write(&config_path, default_config)?;
            tracing::info!("已创建工作空间配置: {}", config_path.display());
            Ok(())
        }
        Some(SubArgs::List) => {
            let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
            ws.first_walk(filter)?;
            let status = ws.status();
            println!("表格列表 (共 {} 个)", status.list_count + status.dict_count + status.class_count + status.enumerate_count);
            println!("{:-<60}", "");
            for list in ws.lists() {
                println!("  [List]    {}", list.name);
            }
            for dict in ws.dicts() {
                println!("  [Dict]    {}", dict.name);
            }
            for class in ws.classes() {
                println!("  [Class]   {}", class.name);
            }
            for enumerate in ws.enumerates() {
                println!("  [Enum]    {}", enumerate.name);
            }
            Ok(())
        }
        _ => {
            let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
            tracing::info!("工作空间根目录: {:?}", ws.config.root);
            
            // 从 generators 列表中查找 Unity 配置
            for generator in &ws.config.generators {
                if let Generator::Unity(unity) = generator {
                    tracing::info!("Unity 加载器启用: {:?}", unity.enable);
                    tracing::info!("Unity 加载器输出: {:?}", unity.output);
                }
            }
            
            tracing::info!("生成器数量: {:?}", ws.config.generators.len());
            
            // 先进行首次遍历，加载表数据
            ws.first_walk(filter)?;
            
            // 然后使用 xcell-generator 模块进行代码生成
            let config = xcell_generator::config::GeneratorConfig::from_project_config(&ws.config);
            tracing::info!("生成产物数量: {:?}", config.products.len());
            
            for product in &config.products {
                tracing::debug!("产物类型: {:?}, 输出目录: {:?}, 启用: {:?}", product.product_type, product.output_dir, product.enabled);
            }
            
            let generator = xcell_generator::Generator::new(config);
            tracing::info!("生成器已创建, 生成器数量: {:?}", generator.generator_count());
            
            tracing::info!("调用 generator.generate()");
            generator.generate(&ws)?;
            tracing::info!("generator.generate() 完成");
            
            if args.watch {
                ws.watcher().await?;
            }
            Ok(())
        }
    };
    pause();
    result
}
