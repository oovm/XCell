use super::{Codegen, CodegenContext};
use dejavu_engine::jit::{vm::DejavuVM, Context};
use std::collections::HashMap;
use std::{fs, path::PathBuf};
use tracing::{debug as tracing_debug, error, info};
use xcell_analyzer::{XClassData, XDictData, XEnumerateData, XListData};
use xcell_core::{XError, XErrorKind, XResult};

/// 动态 DejaVu 代码生成器
///
/// 使用 dejavu-engine 的 VM 进行动态模板渲染
pub struct DynamicDejavuCodegen {
    /// DejaVu 虚拟机
    vm: DejavuVM,
}

impl DynamicDejavuCodegen {
    /// 创建新的动态 DejaVu 代码生成器实例
    pub fn new() -> Self {
        let vm = DejavuVM::new();

        Self { vm }
    }

    /// 将 XClassData 转换为 dejavu Context
    fn class_data_to_context(&self, data: &XClassData, prefix: &str) {
        let class_key = if prefix.is_empty() {
            data.name.clone()
        } else {
            format!("{}.{}", prefix, data.name)
        };

        // 添加类名
        self.vm.global_context_mut().set(&class_key, dejavu_types::Value::String(data.name.clone()));

        // 添加字段
        for (idx, item) in data.items.iter().enumerate() {
            let field_key = format!("{}.items[{}].field", class_key, idx);
            self.vm.global_context_mut().set(&field_key, dejavu_types::Value::String(item.field.clone()));

            let typing_key = format!("{}.items[{}].typing", class_key, idx);
            self.vm.global_context_mut().set(&typing_key, dejavu_types::Value::String(format!("{:?}", item.typing)));

            let default_key = format!("{}.items[{}].default", class_key, idx);
            self.vm.global_context_mut().set(&default_key, dejavu_types::Value::String(format!("{:?}", item.default)));

            // 添加文档注释
            let doc_lines: Vec<dejavu_types::Value> = item
                .document
                .lines()
                .map(|line| dejavu_types::Value::String(line.to_string()))
                .collect();
            let doc_key = format!("{}.items[{}].document", class_key, idx);
            self.vm.global_context_mut().set(&doc_key, dejavu_types::Value::Array(doc_lines));
        }
    }

    /// 将 XDictData 转换为 dejavu Context
    fn dict_data_to_context(&self, data: &XDictData, prefix: &str) {
        let dict_key = if prefix.is_empty() {
            data.name.clone()
        } else {
            format!("{}.{}", prefix, data.name)
        };

        self.vm.global_context_mut().set(&dict_key, dejavu_types::Value::String(data.name.clone()));

        for (idx, (key, value)) in data.items.iter().enumerate() {
            let item_key = format!("{}.items[{}].key", dict_key, idx);
            self.vm.global_context_mut().set(&item_key, dejavu_types::Value::String(key.clone()));

            let value_key = format!("{}.items[{}].value", dict_key, idx);
            self.vm.global_context_mut().set(&value_key, dejavu_types::Value::String(value.clone()));
        }
    }

    /// 将 XEnumerateData 转换为 dejavu Context
    fn enumerate_data_to_context(&self, data: &XEnumerateData, prefix: &str) {
        let enum_key = if prefix.is_empty() {
            data.name.clone()
        } else {
            format!("{}.{}", prefix, data.name)
        };

        self.vm.global_context_mut().set(&enum_key, dejavu_types::Value::String(data.name.clone()));

        if let Some(underlying) = &data.underlying {
            let underlying_key = format!("{}.underlying", enum_key);
            self.vm.global_context_mut().set(&underlying_key, dejavu_types::Value::String(format!("{:?}", underlying)));
        }

        for (idx, item) in data.items.iter().enumerate() {
            let name_key = format!("{}.items[{}].name", enum_key, idx);
            self.vm.global_context_mut().set(&name_key, dejavu_types::Value::String(item.name.clone()));

            let value_key = format!("{}.items[{}].value", enum_key, idx);
            self.vm.global_context_mut().set(&value_key, dejavu_types::Value::String(format!("{:?}", item.value)));

            // 添加文档注释
            let doc_lines: Vec<dejavu_types::Value> = item
                .document
                .lines()
                .map(|line| dejavu_types::Value::String(line.to_string()))
                .collect();
            let doc_key = format!("{}.items[{}].document", enum_key, idx);
            self.vm.global_context_mut().set(&doc_key, dejavu_types::Value::Array(doc_lines));
        }
    }

    /// 将 XListData 转换为 dejavu Context
    fn list_data_to_context(&self, data: &XListData, prefix: &str) {
        let list_key = if prefix.is_empty() {
            data.name.clone()
        } else {
            format!("{}.{}", prefix, data.name)
        };

        self.vm.global_context_mut().set(&list_key, dejavu_types::Value::String(data.name.clone()));

        for (idx, item) in data.items.iter().enumerate() {
            let name_key = format!("{}.items[{}].name", list_key, idx);
            self.vm.global_context_mut().set(&name_key, dejavu_types::Value::String(item.name.clone()));

            let typing_key = format!("{}.items[{}].typing", list_key, idx);
            self.vm.global_context_mut().set(&typing_key, dejavu_types::Value::String(format!("{:?}", item.typing)));
        }
    }

    /// 构建完整的渲染上下文
    fn build_render_context(&mut self, context: &CodegenContext) {
        // 添加配置选项
        for (key, value) in &context.options {
            let full_key = format!("options.{}", key);
            self.vm.global_context_mut().set(&full_key, dejavu_types::Value::String(value.clone()));
        }

        // 添加全局配置
        for (key, value) in &context.global_options {
            let full_key = format!("global.{}", key);
            self.vm.global_context_mut().set(&full_key, dejavu_types::Value::String(value.clone()));
        }

        // 如果存在工作区管理器，添加表数据
        if let Some(workspace) = context.workspace {
            // 添加类表数据
            for class_data in workspace.classes() {
                self.class_data_to_context(class_data, "tables.classes");
            }

            // 添加字典表数据
            for dict_data in workspace.dicts() {
                self.dict_data_to_context(dict_data, "tables.dicts");
            }

            // 添加枚举表数据
            for enum_data in workspace.enumerates() {
                self.enumerate_data_to_context(enum_data, "tables.enums");
            }

            // 添加列表表数据
            for list_data in workspace.lists() {
                self.list_data_to_context(list_data, "tables.lists");
            }
        }
    }

    /// 渲染单个模板
    fn render_template(&mut self, template_name: &str) -> XResult<String> {
        let ctx = self.vm.global_context().clone();
        self.vm
            .render_template(template_name, &ctx)
            .map_err(|e| XError::new(XErrorKind::RuntimeError { message: format!("模板渲染失败: {}", e) }))
    }
}

impl Default for DynamicDejavuCodegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen for DynamicDejavuCodegen {
    /// 生成代码
    ///
    /// # Arguments
    /// * `context` - 代码生成上下文
    ///
    /// # Returns
    /// 生成结果
    fn generate(&self, context: &CodegenContext) -> XResult<()> {
        info!("开始动态 DejaVu 代码生成");
        tracing_debug!("输出目录: {:?}", context.output_dir);

        // 确定模板目录
        let template_dir: String = context.get_option("template_dir", "templates").into();
        let template_dir_path = PathBuf::from(&template_dir);
        info!("模板目录: {:?}", template_dir_path);

        // 如果模板目录不存在，则创建
        if !template_dir_path.exists() {
            info!("模板目录不存在，正在创建...");
            fs::create_dir_all(&template_dir_path).map_err(|e| {
                let error_msg = format!("创建模板目录失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?;
        }

        // 创建新的 VM 实例
        let mut vm = DejavuVM::new();

        // 查找所有模板文件
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
            error!("{}", error_msg);
            return Err(XError::new(XErrorKind::RuntimeError { message: error_msg }));
        }

        info!("找到 {} 个模板文件", template_files.len());

        // 加载所有模板到 VM
        for entry in &template_files {
            let template_path = entry.path();
            let template_name = template_path
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("template");

            info!("加载模板文件: {:?}", template_path);

            // 读取模板内容
            let template_content = fs::read_to_string(&template_path).map_err(|e| {
                let error_msg = format!("读取模板文件失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?;

            // 注册模板到 VM
            vm.parse_and_register(template_name.to_string(), &template_content)
                .map_err(|e| {
                    let error_msg = format!("解析模板失败: {}", e);
                    error!("{}", error_msg);
                    XError::new(XErrorKind::RuntimeError { message: error_msg })
                })?;
        }

        // 构建渲染上下文
        let mut codegen = DynamicDejavuCodegen { vm };
        codegen.build_render_context(context);

        // 为每个模板文件渲染并输出
        for entry in template_files {
            let template_path = entry.path();
            let template_stem = template_path
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("template");

            // 处理带点的文件名，比如 hello.rs.dejavu 应该生成 hello.rs
            let output_filename = if template_stem.contains('.') {
                template_stem.to_string()
            } else {
                format!("{}.rs", template_stem)
            };

            info!("处理模板文件: {:?}", template_path);

            // 渲染模板
            let rendered_content = codegen.render_template(template_stem)?;

            // 生成输出路径
            let output_path = context.output_dir.join(output_filename);
            info!("生成代码文件: {:?}", output_path);

            // 确保输出目录存在
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| {
                        let error_msg = format!("创建输出目录失败: {}", e);
                        error!("{}", error_msg);
                        XError::new(XErrorKind::RuntimeError { message: error_msg })
                    })?;
                }
            }

            // 写入渲染后的内容
            fs::write(&output_path, rendered_content).map_err(|e| {
                let error_msg = format!("写入输出文件失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?;

            info!("模板 {} 处理完成", template_stem);
        }

        info!("动态 DejaVu 代码生成完成");
        Ok(())
    }

    /// 获取生成器名称
    fn name(&self) -> &'static str {
        "dejavu"
    }
}
