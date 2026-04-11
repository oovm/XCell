use std::{fs::File, io::Read, path::Path, sync::Arc};
use xcell_core::{XError, XResult};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static DEFAULT_TEMPLATES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut templates = HashMap::new();
    
    // 默认枚举模板
    templates.insert("BuildEnumerate.ts.dejavu", include_str!("../../templates/BuildEnumerate.ts.dejavu"));
    
    // 默认类模板
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
    
    /// 渲染模板
    ///
    /// # 参数
    /// * `template_name` - 模板名称
    /// * `context` - 模板上下文
    ///
    /// # 返回值
    /// 返回渲染后的内容，成功时返回 Ok(String)，失败时返回 XError
    pub fn render_template(&self, template_name: &str, context: &serde_json::Value) -> XResult<String> {
        let template_content = self.load_template(template_name)?;
        
        // 简单的模板渲染实现
        let mut result = template_content;
        
        // 处理变量替换: <% variable %>
        if let Some(obj) = context.as_object() {
            for (key, value) in obj {
                let placeholder = format!("<% {} %>", key);
                let value_str = value.to_string().trim_matches('"').to_string();
                result = result.replace(&placeholder, &value_str);
            }
        }
        
        // 处理循环结构: <% loop item in items %>
        if let Some(obj) = context.as_object() {
            // 寻找所有循环结构
            while let Some(loop_start) = result.find("<% loop ") {
                // 提取循环变量名和数组名
                let loop_start_str = &result[loop_start..];
                if let Some(loop_end) = loop_start_str.find(" %>") {
                    let loop_command = &loop_start_str[6..loop_end];
                    if let Some(in_idx) = loop_command.find(" in ") {
                        let item_var = loop_command[..in_idx].trim();
                        let array_name = loop_command[in_idx + 4..].trim();
                        
                        // 查找循环结束标记（处理嵌套循环）
                        let end_loop = format!("<% end loop %>");
                        let mut end_idx = None;
                        let mut loop_count = 1;
                        let mut pos = loop_start + 6; // 跳过 "<% loop "
                        
                        while loop_count > 0 && pos < result.len() {
                            if let Some(inner_loop_start) = result[pos..].find("<% loop ") {
                                let inner_pos = pos + inner_loop_start;
                                if let Some(inner_end) = result[inner_pos..].find(" %>") {
                                    loop_count += 1;
                                    pos = inner_pos + inner_end + 2;
                                    continue;
                                }
                            }
                            
                            if let Some(inner_end_loop) = result[pos..].find(&end_loop) {
                                let inner_pos = pos + inner_end_loop;
                                loop_count -= 1;
                                if loop_count == 0 {
                                    end_idx = Some(inner_end_loop);
                                    break;
                                }
                                pos = inner_pos + end_loop.len();
                                continue;
                            }
                            
                            break;
                        }
                        
                        if let Some(end_idx) = end_idx {
                            let end_pos = loop_start + end_idx + end_loop.len();
                            
                            // 提取循环体
                            let before_loop = &result[..loop_start];
                            let loop_start_marker = &loop_start_str[..loop_end + 2]; // "<% loop ... %>"
                            let loop_body_start = loop_start + loop_start_marker.len();
                            let loop_body = &result[loop_body_start..loop_start + end_idx];
                            let after_loop = &result[end_pos..];
                            
                            // 检查数组是否存在
                            if let Some(array_value) = obj.get(array_name) {
                                if let serde_json::Value::Array(array) = array_value {
                                    // 渲染循环内容
                                    let mut loop_content = String::new();
                                    for item in array {
                                        if let serde_json::Value::Object(item_obj) = item {
                                            let mut rendered_item = loop_body.to_string();
                                            
                                            // 替换循环体内的变量
                                            for (key, value) in item_obj {
                                                // 处理 <% item_var.key %> 形式的占位符
                                                let placeholder = format!("<% {}.{} %>", item_var, key);
                                                let value_str = value.to_string().trim_matches('"').to_string();
                                                rendered_item = rendered_item.replace(&placeholder, &value_str);
                                            }
                                            
                                            // 处理 <% item_var %> 形式的占位符（当 item 本身是字符串或其他简单类型时）
                                            if let Some(value) = item.as_str() {
                                                let placeholder = format!("<% {} %>", item_var);
                                                rendered_item = rendered_item.replace(&placeholder, value);
                                            } else if let Some(value) = item.as_f64() {
                                                let placeholder = format!("<% {} %>", item_var);
                                                rendered_item = rendered_item.replace(&placeholder, &value.to_string());
                                            } else if let Some(value) = item.as_bool() {
                                                let placeholder = format!("<% {} %>", item_var);
                                                rendered_item = rendered_item.replace(&placeholder, &value.to_string());
                                            }
                                            
                                            loop_content.push_str(&rendered_item);
                                        }
                                    }
                                    
                                    // 重新组合结果
                                    result = format!("{}{}{}", before_loop, loop_content, after_loop);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(result)
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
            TemplateType::Enumerate => "BuildEnumerate.ts.dejavu",
            TemplateType::Class => "BuildClass.ts.dejavu",
            TemplateType::Manager => "BuildManager.ts.dejavu",
        }
    }
}
