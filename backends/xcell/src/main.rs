use clap::Parser;
use xcell::{SubArgs, TomlSubArgs, XCellArgs, logger, pause};
use xcell_analyzer::{WorkspaceManager, XResult, XError};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> XResult<()> {
    logger();
    let args = XCellArgs::parse();
    let result = match args.command {
        Some(SubArgs::Check) => {
            let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
            ws.first_walk()?;
            Ok(())
        }
        Some(SubArgs::Clear) => {
            let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
            // 清空文件修改时间记录
            ws.file_modification_times.clear();
            println!("Cleared workspace cache");
            Ok(())
        }
        Some(SubArgs::Toml { subcommand }) => match subcommand {
            TomlSubArgs::List { file } => {
                let content = std::fs::read_to_string(&file)?;
                let table: toml::Value = content.parse().map_err(|e| XError::runtime_error(format!("TOML parse error: {}", e)))?;
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
                let mut table: toml::Value = if std::path::Path::new(&file).exists() {
                    let content = std::fs::read_to_string(&file)?;
                    content.parse().map_err(|e| XError::runtime_error(format!("TOML parse error: {}", e)))?
                }
                else {
                    toml::Value::Table(toml::Table::new())
                };
                let fields = table.get_mut("fields").and_then(|v| v.as_array_mut());
                let fields = match fields {
                    Some(fields) => fields,
                    None => {
                        let new_array = toml::Value::Array(vec![]);
                        if let toml::Value::Table(ref mut table) = table {
                            table.insert("fields".to_string(), new_array);
                        }
                        table.get_mut("fields").and_then(|v| v.as_array_mut()).unwrap()
                    }
                };
                let mut field = toml::Value::Table(toml::Table::new());
                if let toml::Value::Table(ref mut field_table) = field {
                    field_table.insert("name".to_string(), toml::Value::String(name.clone()));
                    field_table.insert("type".to_string(), toml::Value::String(r#type));
                    if let Some(comment) = comment {
                        field_table.insert("comment".to_string(), toml::Value::String(comment));
                    }
                    if let Some(default) = default {
                        field_table.insert("default".to_string(), toml::Value::String(default));
                    }
                }
                fields.push(field);
                let content = toml::to_string(&table).map_err(|e| XError::runtime_error(format!("TOML serialize error: {}", e)))?;
                std::fs::write(&file, content)?;
                println!("Added field {} to {}", name, file);
                Ok(())
            }
            TomlSubArgs::Remove { file, name } => {
                let content = std::fs::read_to_string(&file)?;
                let mut table: toml::Value = content.parse().map_err(|e| XError::runtime_error(format!("TOML parse error: {}", e)))?;
                if let Some(fields) = table.get_mut("fields").and_then(|v| v.as_array_mut()) {
                    let initial_len = fields.len();
                    fields.retain(|field| field.get("name").and_then(|v| v.as_str()) != Some(&name));
                    if fields.len() < initial_len {
                        let content = toml::to_string(&table).map_err(|e| XError::runtime_error(format!("TOML serialize error: {}", e)))?;
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
                let mut table: toml::Value = content.parse().map_err(|e| XError::runtime_error(format!("TOML parse error: {}", e)))?;
                if let Some(fields) = table.get_mut("fields").and_then(|v| v.as_array_mut()) {
                    let mut found = false;
                    for field in fields {
                        if let toml::Value::Table(field_table) = field {
                            if field_table.get("name").and_then(|v| v.as_str()) == Some(&name) {
                                if let Some(r#type) = r#type {
                                    field_table.insert("type".to_string(), toml::Value::String(r#type));
                                }
                                if let Some(comment) = comment {
                                    field_table.insert("comment".to_string(), toml::Value::String(comment));
                                }
                                if let Some(default) = default {
                                    field_table.insert("default".to_string(), toml::Value::String(default));
                                }
                                found = true;
                                break;
                            }
                        }
                    }
                    if found {
                        let content = toml::to_string(&table).map_err(|e| XError::runtime_error(format!("TOML serialize error: {}", e)))?;
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
        _ => {
            let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
            println!("Workspace root: {:?}", ws.config.root);
            println!("Unity loader enable: {:?}", ws.config.unity.loader.enable);
            println!("Unity loader output: {:?}", ws.config.unity.loader.output);
            println!("Generators count: {:?}", ws.config.generators.len());
            
            // 先使用 xcell-generator 模块进行代码生成
            let config = xcell_generator::config::GeneratorConfig::from_project_config(&ws.config);
            println!("Generated products count: {:?}", config.products.len());
            
            for product in &config.products {
                println!("Product type: {:?}, output_dir: {:?}, enabled: {:?}", product.product_type, product.output_dir, product.enabled);
            }
            
            let generator = xcell_generator::Generator::new(config);
            println!("Generator created, generator count: {:?}", generator.generator_count());
            
            println!("Calling generator.generate()");
            generator.generate(&ws)?;
            println!("generator.generate() completed");
            
            // 然后再进行首次遍历
            ws.first_walk()?;
            
            if args.watch {
                ws.watcher().await?;
            }
            Ok(())
        }
    };
    pause();
    result
}
