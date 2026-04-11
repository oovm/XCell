//! 统一配置接口
//! 
//! 提供一个统一的配置管理接口，便于其他模块和 crate 访问和使用配置。

use std::path::Path;

use crate::{GeneratorConfig, ProjectConfig};

/// 统一配置管理器
/// 
/// 提供对所有配置类型的统一访问和管理。
pub struct ConfigManager {
    /// 项目配置
    pub project_config: ProjectConfig,
    /// 生成器配置
    pub generator_config: GeneratorConfig,
}

impl ConfigManager {
    /// 从工作空间路径创建配置管理器
    /// 
    /// # Parameters
    /// - `workspace_path`: 工作空间路径
    /// 
    /// # Returns
    /// - 配置管理器实例
    pub fn new(workspace_path: &Path) -> Self {
        let project_config = ProjectConfig::new(workspace_path);
        let generator_config = GeneratorConfig::from_project_config(&project_config);
        
        Self {
            project_config,
            generator_config,
        }
    }
    
    /// 从项目配置更新生成器配置
    pub fn update_generator_config(&mut self) {
        self.generator_config = GeneratorConfig::from_project_config(&self.project_config);
    }
    
    /// 验证所有配置
    pub fn validate(&self) -> Result<(), String> {
        self.generator_config.validate()
    }
}
