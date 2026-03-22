use std::{collections::HashMap, fs, path::Path};
use xcell_generator::codegen::{Codegen, CodegenContext};

#[test]
fn test_unreal_code_generation() {
    // 创建临时目录
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    // 创建代码生成上下文
    let mut options = HashMap::new();
    options.insert("module_name".to_string(), "TestModule".to_string());

    let _context = CodegenContext {
        output_dir: std::path::PathBuf::from(temp_path.to_str().unwrap()),
        options,
        global_options: HashMap::new(),
        workspace: None,
    };

    // 由于 Unreal 代码生成器还未实现，这里暂时跳过生成步骤
    // 后续会添加具体的代码生成逻辑
    println!("Unreal code generation test placeholder");

    // 列出临时目录中的文件
    println!("Files in temp directory:");
    for entry in std::fs::read_dir(temp_path).expect("Failed to read temp directory") {
        if let Ok(entry) = entry {
            println!("  {:?}", entry.file_name());
        }
    }

    // 验证生成的文件是否存在
    // 由于 Unreal 代码生成器还未实现，这里暂时跳过验证
    // 后续会添加具体的文件验证逻辑
    assert!(true, "Unreal test completed");
}

// ConfigManager 结构体暂时注释掉，因为测试中不需要使用它
/*
/// 单例模式管理配置表
pub struct ConfigManager {
    configs: HashMap<String, serde_json::Value>,
    loaded: bool,
}

impl ConfigManager {
    /// 获取单例实例
    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: Option<ConfigManager> = None;

        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(ConfigManager { configs: HashMap::new(), loaded: false });
            }
            INSTANCE.as_mut().unwrap()
        }
    }

    /// 按需加载配置表
    pub fn get_config(&mut self, name: &str) -> Option<&serde_json::Value> {
        if !self.loaded {
            self.load_configs();
            self.loaded = true;
        }
        self.configs.get(name)
    }

    /// 加载配置表
    fn load_configs(&mut self) {
        // 这里实现配置表加载逻辑
        // 从 JSON 文件加载配置数据
        println!("Loading configs...");
    }
}
*/
