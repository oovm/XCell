use super::{Codegen, CodegenContext};
use nargo_template::{DejaVuAdapter, DejaVuFrontend, UnifiedTemplateEngine};
use nargo_types::NargoValue;
use std::collections::HashMap;
use std::{fs, path::PathBuf};
use tracing::{debug as tracing_debug, error, info};
use xcell_analyzer::{XClassData, XDictData, XEnumerateData, XListData};
use xcell_core::{XError, XErrorKind, XResult};
use crate::codegen::core::typescript::{AsTypeScriptType, AsTypeScriptDefault};

/// 动态 DejaVu 代码生成器
///
/// 使用 nargo-template 提供的 DejaVu 适配器进行动态模板渲染
pub struct DynamicDejavuCodegen {
    /// DejaVu 适配器
    adapter: DejaVuAdapter,
}

impl DynamicDejavuCodegen {
    /// 创建新的动态 DejaVu 代码生成器实例
    pub fn new() -> Self {
        let adapter = DejaVuAdapter::new(DejaVuFrontend::new());

        Self {
            adapter
        }
    }

    /// 将 XClassData 转换为 NargoValue
    fn class_data_to_value(&self, data: &XClassData) -> NargoValue {
        let mut class_obj = HashMap::new();
        class_obj.insert("name".to_string(), NargoValue::String(data.name.clone()));

        let mut items = Vec::new();
        for item in &data.items {
            let mut item_obj = HashMap::new();
            item_obj.insert("field".to_string(), NargoValue::String(item.field.clone()));
            item_obj.insert("typing".to_string(), NargoValue::String(format!("{:?}", item.typing)));
            item_obj.insert("default".to_string(), NargoValue::String(format!("{:?}", item.default)));

            let doc_lines: Vec<NargoValue> = item
                .document
                .lines()
                .into_iter()
                .map(|line| NargoValue::String(line.to_string()))
                .collect();
            item_obj.insert("document".to_string(), NargoValue::Array(doc_lines));

            items.push(NargoValue::Object(item_obj));
        }
        class_obj.insert("items".to_string(), NargoValue::Array(items));

        NargoValue::Object(class_obj)
    }

    /// 将 XDictData 转换为 NargoValue
    fn dict_data_to_value(&self, data: &XDictData) -> NargoValue {
        let mut dict_obj = HashMap::new();
        dict_obj.insert("name".to_string(), NargoValue::String(data.name.clone()));

        let mut items = Vec::new();
        for (key, value) in &data.mapping {
            let mut item_obj = HashMap::new();
            item_obj.insert("key".to_string(), NargoValue::String(key.clone()));
            item_obj.insert("value".to_string(), NargoValue::String(value.key.clone()));
            items.push(NargoValue::Object(item_obj));
        }
        dict_obj.insert("items".to_string(), NargoValue::Array(items));

        NargoValue::Object(dict_obj)
    }

    /// 将 XEnumerateData 转换为 NargoValue
    fn enumerate_data_to_value(&self, data: &XEnumerateData) -> NargoValue {
        let mut enum_obj = HashMap::new();
        enum_obj.insert("name".to_string(), NargoValue::String(data.name.clone()));

        if !data.headers.is_empty() {
            let headers_value: Vec<NargoValue> = data.headers.iter().map(|h| NargoValue::String(h.field_name.clone())).collect();
            enum_obj.insert("headers".to_string(), NargoValue::Array(headers_value));
        }

        let mut items = Vec::new();
        for line in &data.lines {
            let mut item_obj = HashMap::new();
            item_obj.insert("line".to_string(), NargoValue::String(line.key.clone()));
            item_obj.insert("name".to_string(), NargoValue::String(line.key.clone()));
            item_obj.insert("value".to_string(), NargoValue::String(line.key.clone()));
            items.push(NargoValue::Object(item_obj));
        }
        enum_obj.insert("items".to_string(), NargoValue::Array(items));

        NargoValue::Object(enum_obj)
    }

    /// 将 XListData 转换为 NargoValue
    fn list_data_to_value(&self, data: &XListData) -> NargoValue {
        let mut list_obj = HashMap::new();
        list_obj.insert("name".to_string(), NargoValue::String(data.name.clone()));

        let mut items = Vec::new();
        for (key, value) in &data.mapping {
            let mut item_obj = HashMap::new();
            item_obj.insert("name".to_string(), NargoValue::String(key.to_string()));
            item_obj.insert("typing".to_string(), NargoValue::String(value.key.clone()));
            items.push(NargoValue::Object(item_obj));
        }
        list_obj.insert("items".to_string(), NargoValue::Array(items));

        NargoValue::Object(list_obj)
    }

    /// 构建完整的渲染上下文
    fn build_render_context(&self, context: &CodegenContext) -> NargoValue {
        let mut root_obj = HashMap::new();

        // 添加编译器版本
        root_obj.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));

        // 如果存在工作区管理器，添加表数据
        if let Some(workspace) = context.workspace {
            // 处理类表数据
            let class_items = workspace.classes()
                .map(|t| format!("{}{}", t.name, context.get_option("suffix_table", "Table")))
                .chain(workspace.dicts().map(|t| format!("{}{}", t.name, context.get_option("suffix_table", "Table"))))
                .chain(workspace.lists().map(|t| format!("{}{}", t.name, context.get_option("suffix_table", "Table"))))
                .collect::<Vec<String>>();
            
            let tables_value: Vec<NargoValue> = class_items.iter().map(|table| {
                let mut table_data = HashMap::new();
                table_data.insert("typing".to_string(), NargoValue::String(table.clone()));
                table_data.insert("private_name".to_string(), NargoValue::String(table.to_lowercase()));
                table_data.insert("public_name".to_string(), NargoValue::String(format!("get{}", table)));
                NargoValue::Object(table_data)
            }).collect();
            root_obj.insert("tables".to_string(), NargoValue::Array(tables_value));

            // 为每个类表构建单独的上下文
            for class_data in workspace.classes() {
                let class_name = class_data.name.clone();
                let table_name = format!("{}{}", class_name, context.get_option("suffix_table", "Table"));
                
                // 构建类字段
                let class_fields_value: Vec<NargoValue> = class_data.items.iter().map(|item| {
                    let default = item.typing.as_typescript_default();
                    let mut field_data = HashMap::new();
                    field_data.insert("document".to_string(), NargoValue::Array(
                        item.document.lines().into_iter().map(|doc| NargoValue::String(doc)).collect()
                    ));
                    field_data.insert("name".to_string(), NargoValue::String(item.field.clone()));
                    field_data.insert("typing".to_string(), NargoValue::String(item.typing.as_typescript_type()));
                    field_data.insert("has_default".to_string(), NargoValue::Bool(!default.is_empty()));
                    field_data.insert("default".to_string(), NargoValue::String(default));
                    NargoValue::Object(field_data)
                }).collect();
                
                // 添加类相关的上下文
                root_obj.insert(format!("{}_class_name", class_name).to_string(), NargoValue::String(class_name.clone()));
                root_obj.insert(format!("{}_table_name", class_name).to_string(), NargoValue::String(table_name));
                root_obj.insert(format!("{}_class_fields", class_name).to_string(), NargoValue::Array(class_fields_value));
            }

            // 处理枚举表数据
            for enum_data in workspace.enumerates() {
                let enum_name = enum_data.name.clone();
                
                // 构建枚举项
                let enumerate_ids_value: Vec<NargoValue> = enum_data.lines.iter().map(|line| {
                    let mut item_data = HashMap::new();
                    item_data.insert("key".to_string(), NargoValue::String(line.key.clone()));
                    item_data.insert("value".to_string(), NargoValue::String(line.id.to_string()));
                    item_data.insert("document".to_string(), NargoValue::Array(
                        vec![NargoValue::String(String::new())]
                    ));
                    NargoValue::Object(item_data)
                }).collect();
                
                // 添加枚举相关的上下文
                root_obj.insert(format!("{}_class_name", enum_name).to_string(), NargoValue::String(enum_name.clone()));
                root_obj.insert(format!("{}_enumerate_ids", enum_name).to_string(), NargoValue::Array(enumerate_ids_value));
            }
        }

        // 添加配置选项
        let manager_name = context.get_option("manager_name", "XCellManager");
        let instance_name = context.get_option("instance_name", "xcell");
        root_obj.insert("class_name".to_string(), NargoValue::String(manager_name.clone()));
        root_obj.insert("instance_name".to_string(), NargoValue::String(instance_name));
        root_obj.insert("manager_name".to_string(), NargoValue::String(manager_name));

        // 添加数据路径
        let storage = context.get_option("storage", "src/table/data");
        let table_data_path = if storage.is_empty() {
            "src/table/data/".to_string()
        } else if storage.ends_with('/') || storage.ends_with('\\') {
            storage
        } else {
            format!("{}/", storage)
        };
        root_obj.insert("table_data_path".to_string(), NargoValue::String(table_data_path));

        NargoValue::Object(root_obj)
    }
}

impl Default for DynamicDejavuCodegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen for DynamicDejavuCodegen {
    /// 生成代码
    fn generate(&self, context: &CodegenContext) -> XResult<()> {
        info!("开始动态 DejaVu 代码生成");
        tracing_debug!("输出目录: {:?}", context.output_dir);

        let template_dir: String = context.get_option("template_dir", "templates").into();
        
        let template_dir_path = if let Some(workspace) = context.workspace {
            let path = std::path::PathBuf::from(&template_dir);
            if path.is_absolute() {
                path
            } else {
                workspace.config.root.join(&template_dir)
            }
        } else {
            PathBuf::from(&template_dir)
        };
        info!("模板目录: {:?}", template_dir_path);

        if !template_dir_path.exists() {
            info!("模板目录不存在，正在创建...");
            fs::create_dir_all(&template_dir_path).map_err(|e| {
                let error_msg = format!("创建模板目录失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?;
        }

        let mut adapter = DejaVuAdapter::new(DejaVuFrontend::new());

        let template_files: Vec<_> = fs::read_dir(&template_dir_path)
            .map_err(|e| {
                let error_msg = format!("读取模板目录失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let path = entry.path();
                let ext = path.extension().and_then(|ext| ext.to_str());
                ext == Some("dejavu") || ext == Some("dj")
            })
            .collect();

        if template_files.is_empty() {
            let error_msg = format!("模板目录中没有找到 .dejavu 或 .dj 文件: {:?}", template_dir_path);
            info!("{}", error_msg);
            info!("跳过 dejavu 代码生成");
            return Ok(());
        }

        info!("找到 {} 个模板文件", template_files.len());

        for entry in &template_files {
            let template_path = entry.path();
            let template_name = template_path
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("template")
                .to_string();

            let template_content = fs::read_to_string(&template_path).map_err(|e| {
                let error_msg = format!("读取模板文件失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?;

            adapter.register_template(&template_name, &template_content)
                .map_err(|e| {
                    let error_msg = format!("注册模板失败: {}", e);
                    error!("{}", error_msg);
                    XError::new(XErrorKind::RuntimeError { message: error_msg })
                })?;
        }

        let suffix_table = context.get_option("suffix_table", "Table");

        if let Some(workspace) = context.workspace {
            let classes: Vec<_> = workspace.classes().collect();
            let enumerates: Vec<_> = workspace.enumerates().collect();
            info!("工作区中有 {} 个类和 {} 个枚举", classes.len(), enumerates.len());
            
            for class_data in classes {
                let class_name = class_data.name.clone();
                let table_name = format!("{}{}", class_name, suffix_table);
                
                let class_fields_value: Vec<NargoValue> = class_data.items.iter().map(|item| {
                    let default = item.typing.as_typescript_default();
                    let mut field_data = HashMap::new();
                    field_data.insert("document".to_string(), NargoValue::Array(
                        item.document.lines().into_iter().map(|doc| NargoValue::String(doc)).collect()
                    ));
                    field_data.insert("name".to_string(), NargoValue::String(item.field.clone()));
                    field_data.insert("typing".to_string(), NargoValue::String(item.typing.as_typescript_type()));
                    field_data.insert("has_default".to_string(), NargoValue::Bool(!default.is_empty()));
                    field_data.insert("default".to_string(), NargoValue::String(default));
                    NargoValue::Object(field_data)
                }).collect();

                let class_document: Vec<NargoValue> = vec![NargoValue::String(format!("{} 表数据类", class_name))];

                let mut render_context = HashMap::new();
                render_context.insert("class_name".to_string(), NargoValue::String(class_name.clone()));
                render_context.insert("table_name".to_string(), NargoValue::String(table_name.clone()));
                render_context.insert("class_fields".to_string(), NargoValue::Array(class_fields_value));
                render_context.insert("class_document".to_string(), NargoValue::Array(class_document));
                render_context.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));

                let output_filename = format!("{}{}.ts", class_name, suffix_table);
                let output_path = context.output_dir.join(&output_filename);
                info!("生成类文件: {:?}", output_path);

                if let Some(parent) = output_path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent).map_err(|e| {
                            let error_msg = format!("创建输出目录失败: {}", e);
                            error!("{}", error_msg);
                            XError::new(XErrorKind::RuntimeError { message: error_msg })
                        })?;
                    }
                }

                let rendered_content = adapter.render("BuildClass.ts", &NargoValue::Object(render_context))
                    .map_err(|e| {
                        let error_msg = format!("模板渲染失败: {}", e);
                        error!("{}", error_msg);
                        XError::new(XErrorKind::RuntimeError { message: error_msg })
                    })?;

                fs::write(&output_path, rendered_content).map_err(|e| {
                    let error_msg = format!("写入输出文件失败: {}", e);
                    error!("{}", error_msg);
                    XError::new(XErrorKind::RuntimeError { message: error_msg })
                })?;
            }

            for enum_data in workspace.enumerates() {
                let enum_name = enum_data.name.clone();
                
                let enumerate_ids_value: Vec<NargoValue> = enum_data.lines.iter().map(|line| {
                    let mut item_data = HashMap::new();
                    item_data.insert("key".to_string(), NargoValue::String(line.key.clone()));
                    item_data.insert("value".to_string(), NargoValue::String(line.id.to_string()));
                    item_data.insert("document".to_string(), NargoValue::Array(
                        vec![NargoValue::String(String::new())]
                    ));
                    NargoValue::Object(item_data)
                }).collect();

                let mut render_context = HashMap::new();
                render_context.insert("class_name".to_string(), NargoValue::String(enum_name.clone()));
                render_context.insert("enumerate_ids".to_string(), NargoValue::Array(enumerate_ids_value));
                render_context.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));

                let output_filename = format!("{}{}.ts", enum_name, suffix_table);
                let output_path = context.output_dir.join(&output_filename);
                info!("生成枚举文件: {:?}", output_path);

                if let Some(parent) = output_path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent).map_err(|e| {
                            let error_msg = format!("创建输出目录失败: {}", e);
                            error!("{}", error_msg);
                            XError::new(XErrorKind::RuntimeError { message: error_msg })
                        })?;
                    }
                }

                let rendered_content = adapter.render("BuildEnumerate.ts", &NargoValue::Object(render_context))
                    .map_err(|e| {
                        let error_msg = format!("模板渲染失败: {}", e);
                        error!("{}", error_msg);
                        XError::new(XErrorKind::RuntimeError { message: error_msg })
                    })?;

                fs::write(&output_path, rendered_content).map_err(|e| {
                    let error_msg = format!("写入输出文件失败: {}", e);
                    error!("{}", error_msg);
                    XError::new(XErrorKind::RuntimeError { message: error_msg })
                })?;
            }

            let manager_name = context.get_option("manager_name", "XCellManager");
            let instance_name = context.get_option("instance_name", "xcell");
            let storage = context.get_option("storage", "src/table/data");
            let table_data_path = if storage.is_empty() {
                "src/table/data/".to_string()
            } else if storage.ends_with('/') || storage.ends_with('\\') {
                storage
            } else {
                format!("{}/", storage)
            };

            let class_items: Vec<String> = workspace.classes()
                .map(|t| format!("{}{}", t.name, suffix_table))
                .chain(workspace.dicts().map(|t| format!("{}{}", t.name, suffix_table)))
                .chain(workspace.lists().map(|t| format!("{}{}", t.name, suffix_table)))
                .collect();

            let tables_value: Vec<NargoValue> = class_items.iter().map(|table| {
                let mut table_data = HashMap::new();
                table_data.insert("typing".to_string(), NargoValue::String(table.clone()));
                table_data.insert("private_name".to_string(), NargoValue::String(table.to_lowercase()));
                table_data.insert("public_name".to_string(), NargoValue::String(format!("get{}", table)));
                NargoValue::Object(table_data)
            }).collect();

            let mut render_context = HashMap::new();
            render_context.insert("manager_name".to_string(), NargoValue::String(manager_name.clone()));
            render_context.insert("instance_name".to_string(), NargoValue::String(instance_name));
            render_context.insert("table_data_path".to_string(), NargoValue::String(table_data_path));
            render_context.insert("tables".to_string(), NargoValue::Array(tables_value));
            render_context.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));

            let output_filename = format!("{}.ts", manager_name);
            let output_path = context.output_dir.join(&output_filename);
            info!("生成管理器文件: {:?}", output_path);

            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| {
                        let error_msg = format!("创建输出目录失败: {}", e);
                        error!("{}", error_msg);
                        XError::new(XErrorKind::RuntimeError { message: error_msg })
                    })?;
                }
            }

            let rendered_content = adapter.render("BuildManager.ts", &NargoValue::Object(render_context))
                .map_err(|e| {
                    let error_msg = format!("模板渲染失败: {}", e);
                    error!("{}", error_msg);
                    XError::new(XErrorKind::RuntimeError { message: error_msg })
                })?;

            fs::write(&output_path, rendered_content).map_err(|e| {
                let error_msg = format!("写入输出文件失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?;
        }

        info!("动态 DejaVu 代码生成完成");
        Ok(())
    }

    fn name(&self) -> &'static str {
        "dejavu"
    }
}
