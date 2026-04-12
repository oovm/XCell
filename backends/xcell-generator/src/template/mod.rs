use std::{fs::File, io::Read, path::Path, sync::Arc};
use xcell_core::{XError, XResult};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use nargo_template::{DejaVuAdapter, DejaVuFrontend, UnifiedTemplateEngine};
use nargo_types::NargoValue;

static DEFAULT_TEMPLATES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut templates = HashMap::new();
    
    // 默认枚举模板
    templates.insert("BuildEnumerate.ts.dejavu", include_str!("../../templates/BuildEnumerate.ts.dejavu"));
    
    // 默认数据接口模板
    templates.insert("BuildData.ts.dejavu", include_str!("../../templates/BuildData.ts.dejavu"));
    
    // 默认表加载器模板
    templates.insert("BuildTable.ts.dejavu", include_str!("../../templates/BuildTable.ts.dejavu"));
    
    // 默认字典表模板 (Item + Table 静态格式)
    templates.insert("BuildDictTable.ts.dejavu", include_str!("../../templates/BuildDictTable.ts.dejavu"));
    
    // 默认类模板 (保留向后兼容)
    templates.insert("BuildClass.ts.dejavu", include_str!("../../templates/BuildClass.ts.dejavu"));
    
    // 默认管理器模板
    templates.insert("BuildManager.ts.dejavu", include_str!("../../templates/BuildManager.ts.dejavu"));
    
    templates
});

/// 模板加载器
///
/// 负责从目录加载模板文件，如果找不到则使用默认模板
#[derive(Clone)]
pub struct TemplateLoader {
    template_dir: Option<Arc<Path>>,
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
        let template_dir = template_dir.map(Arc::from);
        
        Ok(Self {
            template_dir,
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
            // 尝试加载带 .dejavu 后缀的模板文件
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
            
            // 尝试加载不带后缀的模板文件
            let template_path_no_ext = template_dir.join(template_name.replace(".dejavu", ""));
            if template_path_no_ext.exists() {
                let mut file = File::open(&template_path_no_ext).map_err(|e| {
                    XError::runtime_error(format!("无法打开模板文件 {}: {:?}", template_path_no_ext.display(), e))
                })?;
                
                let mut content = String::new();
                file.read_to_string(&mut content).map_err(|e| {
                    XError::runtime_error(format!("无法读取模板文件 {}: {:?}", template_path_no_ext.display(), e))
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
    
    /// 使用 DejaVuAdapter 渲染模板
    ///
    /// # 参数
    /// * `template_name` - 模板名称
    /// * `context` - 模板上下文
    ///
    /// # 返回值
    /// 返回渲染后的内容，成功时返回 Ok(String)，失败时返回 XError
    pub fn render_with_dejavu(&self, template_name: &str, context: &NargoValue) -> XResult<String> {
        let mut adapter = DejaVuAdapter::new(DejaVuFrontend::new());

        for (name, content) in DEFAULT_TEMPLATES.iter() {
            adapter.register_template(name, content).map_err(|e| {
                XError::runtime_error(format!("注册默认模板 {} 失败: {}", name, e))
            })?;
        }

        if let Some(template_dir) = &self.template_dir {
            let entries = std::fs::read_dir(template_dir).map_err(|e| {
                XError::runtime_error(format!("读取模板目录失败: {:?}", e))
            })?;

            for entry in entries {
                let entry = entry.map_err(|e| {
                    XError::runtime_error(format!("读取目录条目失败: {:?}", e))
                })?;
                let path = entry.path();

                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.ends_with(".dejavu") {
                            let mut file = File::open(&path).map_err(|e| {
                                XError::runtime_error(format!("无法打开模板文件 {}: {:?}", path.display(), e))
                            })?;
                            let mut content = String::new();
                            file.read_to_string(&mut content).map_err(|e| {
                                XError::runtime_error(format!("无法读取模板文件 {}: {:?}", path.display(), e))
                            })?;
                            adapter.register_template(file_name, &content).map_err(|e| {
                                XError::runtime_error(format!("注册模板 {} 失败: {}", file_name, e))
                            })?;
                        }
                    }
                }
            }
        }

        adapter.render(template_name, context).map_err(|e| {
            XError::runtime_error(format!("渲染模板 {} 失败: {}", template_name, e))
        })
    }
}

/// 模板类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateType {
    /// 枚举模板
    Enumerate,
    /// 数据接口模板
    Data,
    /// 表加载器模板
    Table,
    /// 字典表模板 (Item + Table 静态格式)
    DictTable,
    /// 类模板 (保留向后兼容)
    Class,
    /// 管理器模板
    Manager,
}

impl TemplateType {
    /// 获取模板文件名
    pub fn file_name(&self) -> &'static str {
        match self {
            TemplateType::Enumerate => "BuildEnumerate.ts.dejavu",
            TemplateType::Data => "BuildData.ts.dejavu",
            TemplateType::Table => "BuildTable.ts.dejavu",
            TemplateType::DictTable => "BuildDictTable.ts.dejavu",
            TemplateType::Class => "BuildClass.ts.dejavu",
            TemplateType::Manager => "BuildManager.ts.dejavu",
        }
    }
}
