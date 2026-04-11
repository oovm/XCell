use std::{fs::File, io::Read, path::Path, sync::Arc};
use xcell_core::{XError, XResult};
use once_cell::sync::Lazy;
use nargo_template::{DejaVuAdapter, UnifiedTemplateEngine};
use std::collections::HashMap;

static DEFAULT_TEMPLATES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut templates = HashMap::new();
    
    // 默认枚举模板
    templates.insert("enumerate.ts", include_str!("templates/enumerate.ts.dejavu"));
    
    // 默认类模板
    templates.insert("class.ts", include_str!("templates/class.ts.dejavu"));
    
    // 默认管理器模板
    templates.insert("manager.ts", include_str!("templates/manager.ts.dejavu"));
    
    templates
});

/// 模板加载器
///
/// 负责从目录加载模板文件，如果找不到则使用默认模板
#[derive(Clone)]
pub struct TemplateLoader {
    template_dir: Option<Arc<Path>>,
    engine: Arc<DejaVuAdapter>,
}

impl TemplateLoader {
    /// 创建新的模板加载器
    ///
    /// # 参数
    /// * `template_dir` - 模板目录路径，如果为 None 则使用默认模板
    ///
    /// # 返回值
    /// 返回模板加载器实例
    pub fn new(template_dir: Option<&Path>) -> XResult<Self> {
        let engine = DejaVuAdapter::new();
        let template_dir = template_dir.map(Arc::from);
        
        Ok(Self {
            template_dir,
            engine: Arc::new(engine),
        })
    }
    
    /// 加载模板
    ///
    /// # 参数
    /// * `template_name` - 模板名称
    ///
    /// # 返回值
    /// 返回模板内容，成功时返回 Ok(String)，失败时返回 XError
    pub fn load_template(&self, template_name: &str) -> XResult<String> {
        // 尝试从自定义模板目录加载
        if let Some(template_dir) = &self.template_dir {
            let template_path = template_dir.join(template_name);
            if template_path.exists() {
                let mut file = File::open(&template_path).map_err(|e| {
                    XError::runtime_error(format!("无法打开模板文件 {}: {:?}", template_path.display(), e))
                })?;
                
                let mut content = String::new();
                file.read_to_string(&mut content).map_err(|e| {
                    XError::runtime_error(format!("无法读取模板文件 {}: {:?}", template_path.display(), e))
                })?;
                
                return Ok(content);
            }
        }
        
        // 如果自定义模板不存在，使用默认模板
        if let Some(default_template) = DEFAULT_TEMPLATES.get(template_name) {
            Ok(default_template.to_string())
        } else {
            Err(XError::runtime_error(format!("找不到模板: {}", template_name)))
        }
    }
    
    /// 渲染模板
    ///
    /// # 参数
    /// * `template_name` - 模板名称
    /// * `context` - 模板上下文
    ///
    /// # 返回值
    /// 返回渲染后的内容，成功时返回 Ok(String)，失败时返回 XError
    pub fn render_template(&self, template_name: &str, context: &nargo_types::NargoValue) -> XResult<String> {
        let template_content = self.load_template(template_name)?;
        
        let mut engine = DejaVuAdapter::new();
        engine.register_template(template_name, &template_content).map_err(|e| {
            XError::runtime_error(format!("模板注册错误: {}", e))
        })?;
        
        engine.render(template_name, context).map_err(|e| {
            XError::runtime_error(format!("模板渲染错误: {}", e))
        })
    }
}

/// 模板类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateType {
    /// 枚举模板
    Enumerate,
    /// 类模板
    Class,
    /// 管理器模板
    Manager,
}

impl TemplateType {
    /// 获取模板文件名
    pub fn file_name(&self) -> &'static str {
        match self {
            TemplateType::Enumerate => "enumerate.ts",
            TemplateType::Class => "class.ts",
            TemplateType::Manager => "manager.ts",
        }
    }
}
