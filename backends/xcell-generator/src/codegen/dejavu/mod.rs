use super::{Codegen, CodegenContext};
use nargo_template::{DejaVuAdapter, UnifiedTemplateEngine};
use nargo_types::NargoValue;
use std::collections::HashMap;
use std::{fs, path::PathBuf};
use tracing::{debug as tracing_debug, error, info};
use xcell_analyzer::{XClassData, XDictData, XEnumerateData, XListData};
use xcell_core::{XError, XErrorKind, XResult};

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
        let adapter = DejaVuAdapter::new();

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

        // 添加配置选项
        let mut options_obj = HashMap::new();
        for (key, value) in &context.options {
            options_obj.insert(key.clone(), NargoValue::String(value.clone()));
        }
        root_obj.insert("options".to_string(), NargoValue::Object(options_obj));

        // 添加全局配置
        let mut global_obj = HashMap::new();
        for (key, value) in &context.global_options {
            global_obj.insert(key.clone(), NargoValue::String(value.clone()));
        }
        root_obj.insert("global".to_string(), NargoValue::Object(global_obj));

        // 如果存在工作区管理器，添加表数据
        if let Some(workspace) = context.workspace {
            let mut tables_obj = HashMap::new();

            // 添加类表数据
            let mut classes = Vec::new();
            for class_data in workspace.classes() {
                classes.push(self.class_data_to_value(class_data));
            }
            tables_obj.insert("classes".to_string(), NargoValue::Array(classes));

            // 添加字典表数据
            let mut dicts = Vec::new();
            for dict_data in workspace.dicts() {
                dicts.push(self.dict_data_to_value(dict_data));
            }
            tables_obj.insert("dicts".to_string(), NargoValue::Array(dicts));

            // 添加枚举表数据
            let mut enums = Vec::new();
            for enum_data in workspace.enumerates() {
                enums.push(self.enumerate_data_to_value(enum_data));
            }
            tables_obj.insert("enums".to_string(), NargoValue::Array(enums));

            // 添加列表表数据
            let mut lists = Vec::new();
            for list_data in workspace.lists() {
                lists.push(self.list_data_to_value(list_data));
            }
            tables_obj.insert("lists".to_string(), NargoValue::Array(lists));

            root_obj.insert("tables".to_string(), NargoValue::Object(tables_obj));
        }

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

        // 创建新的 DejaVu 适配器实例
        let mut adapter = DejaVuAdapter::new();

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
            info!("{}", error_msg);
            info!("跳过 dejavu 代码生成");
            return Ok(());
        }

        info!("找到 {} 个模板文件", template_files.len());

        // 加载所有模板到适配器
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

            // 注册模板到适配器
            adapter.register_template(template_name, &template_content)
                .map_err(|e| {
                    let error_msg = format!("注册模板失败: {}", e);
                    error!("{}", error_msg);
                    XError::new(XErrorKind::RuntimeError { message: error_msg })
                })?;
        }

        // 构建渲染上下文
        let render_context = self.build_render_context(context);

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
            let rendered_content = adapter.render(template_stem, &render_context)
                .map_err(|e| {
                    let error_msg = format!("模板渲染失败: {}", e);
                    error!("{}", error_msg);
                    XError::new(XErrorKind::RuntimeError { message: error_msg })
                })?;

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
