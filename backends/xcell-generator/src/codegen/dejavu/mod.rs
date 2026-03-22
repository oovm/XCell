use super::{Codegen, CodegenContext};
use std::{fs, path::PathBuf};
use tracing::{debug as tracing_debug, error, info};
use xcell_core::{XError, XErrorKind, XResult};

/// Dejavu 静态生成器
///
/// 负责将 Dejavu 模板编译为 Rust 代码，并提供渲染功能
pub struct DejavuCodegen {
    // Dejavu 生成配置
}

impl DejavuCodegen {
    /// 创建新的 Dejavu 代码生成器实例
    pub fn new() -> Self {
        Self {}
    }

    /// 渲染模板
    ///
    /// # Arguments
    /// * `template_name` - 模板名称
    /// * `ctx` - 渲染上下文
    ///
    /// # Returns
    /// 渲染后的字符串
    pub fn render_template(&self, template_name: &str, _ctx: &dejavu_types::values::Context) -> XResult<String> {
        info!("渲染模板: {}", template_name);

        // 这里可以根据需要实现模板渲染逻辑
        // 由于模板是在编译时生成的，实际的渲染会在生成的代码中进行
        Ok(format!("Template {} rendered with context", template_name))
    }
}

impl Codegen for DejavuCodegen {
    /// 生成代码
    ///
    /// # Arguments
    /// * `context` - 代码生成上下文
    ///
    /// # Returns
    /// 生成结果
    fn generate(&self, context: &CodegenContext) -> XResult<()> {
        info!("开始 Dejavu 代码生成");
        tracing_debug!("输出目录: {:?}", context.output_dir);

        // 定义模板目录路径
        let template_dir = PathBuf::from("templates");
        info!("模板目录: {:?}", template_dir);

        // 确保模板目录存在
        if !template_dir.exists() {
            info!("模板目录不存在，正在创建...");
            match fs::create_dir_all(&template_dir) {
                Ok(_) => info!("模板目录创建成功: {:?}", template_dir),
                Err(e) => {
                    let error_msg = format!("创建模板目录失败: {}", e);
                    error!("{}", error_msg);
                    return Err(XError::new(XErrorKind::RuntimeError { message: error_msg }));
                }
            }
        }

        // 查找模板文件
        let template_files = fs::read_dir(&template_dir)
            .map_err(|e| {
                let error_msg = format!("读取模板目录失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?
            .filter(|entry| {
                entry.as_ref().map_or(false, |e| {
                    let path = e.path();
                    let ext = path.extension().and_then(|ext| ext.to_str());
                    ext == Some("dj") || ext == Some("dejavu")
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                let error_msg = format!("遍历模板目录失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?;

        if template_files.is_empty() {
            let error_msg = format!("模板目录中没有找到 .dj 或 .dejavu 文件: {:?}", template_dir);
            error!("{}", error_msg);
            return Err(XError::new(XErrorKind::RuntimeError { message: error_msg }));
        }

        info!("找到 {} 个模板文件", template_files.len());

        // 为每个模板生成代码
        for entry in template_files {
            let template_path = entry.path();
            let template_name = template_path.file_stem().unwrap_or_default().to_str().unwrap_or("template");
            let template_ext = template_path.extension().and_then(|ext| ext.to_str()).unwrap_or("dj");

            // 处理带点的文件名，比如 hello.rs.dejavu 应该生成 hello.rs
            let output_filename =
                if template_name.contains(".") { template_name.to_string() } else { format!("{}.rs", template_name) };
            info!("处理模板文件: {:?}", template_path);

            // 生成使用 #[derive(Template)] 的 Rust 代码
            let output_path = context.output_dir.join(output_filename);
            info!("生成代码文件: {:?}", output_path);

            // 生成结构体名称，去除文件名中的扩展名
            let struct_name = template_name.split('.').next().unwrap_or(template_name);

            // 生成代码内容
            let code_content = format!(
                "#![allow(unused_imports)]\n\nuse dejavu_macros::Template;\nuse dejavu_types::values::Context;\n\n/// {} 模板\n/// \n/// 自动生成的模板结构体，使用 #[derive(Template)] 宏编译\n#[derive(Template)]\n#[template(path = \"templates/{}.{}\")]\npub struct {}Template;\n\nimpl {}Template {{\n    /// 创建新的模板实例\n    pub fn new() -> Self {{\n        Self\n    }}\n}}\n",
                struct_name, template_name, template_ext, struct_name, struct_name
            );

            // 写入生成的代码文件
            fs::write(&output_path, code_content).map_err(|e| {
                let error_msg = format!("写入输出文件失败: {}", e);
                error!("{}", error_msg);
                XError::new(XErrorKind::RuntimeError { message: error_msg })
            })?;

            info!("模板 {} 处理完成", template_name);
        }

        info!("Dejavu 代码生成完成");
        Ok(())
    }

    /// 获取生成器名称
    fn name(&self) -> &'static str {
        "dejavu"
    }
}
