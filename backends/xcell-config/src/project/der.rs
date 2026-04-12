use super::*;
use crate::cocos::{CocosStorage, CocosJsonConfig};
use crate::unity::{UnityStorage, UnityXluaConfig};
use serde::{Deserialize, Deserializer};
use oak_json::JsonValueNode as JsonValue;

#[derive(Debug, Deserialize, Serialize)]
struct CocosHelper {
    storage: Option<CocosStorage>,
    development: Option<CocosStorage>,
}

#[derive(Deserialize, Default)]
struct CocosLoaderHelper {
    enable: Option<bool>,
    project: Option<String>,
    output: Option<String>,
    namespace: Option<String>,
    manager_name: Option<String>,
    suffix_table: Option<String>,
    instance_name: Option<String>,
    table_data_path: Option<String>,
}

#[derive(Deserialize)]
struct GeneratorHelper {
    enable: Option<bool>,
    project: Option<String>,
    loader: Option<String>,
    loader_template: Option<String>,
    storage: Option<String>,
    storage_type: Option<String>,
    storage_debug_type: Option<String>,
    #[serde(rename = "type")]
    r#type: String,
}

impl<'de> Deserialize<'de> for Generator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 解析配置文件中的生成器配置
        let helper = GeneratorHelper::deserialize(deserializer)?;
        let type_name = helper.r#type.to_ascii_lowercase();

        match type_name.as_str() {
            "cocos" => {
                let mut cocos_codegen = CocosCodegen::default();
                
                // 使用配置文件中的值
                if let Some(enable) = helper.enable {
                    cocos_codegen.enable = enable;
                }
                if let Some(project) = helper.project {
                    cocos_codegen.project = project;
                }
                if let Some(loader) = helper.loader {
                    cocos_codegen.output = loader;
                }
                if let Some(loader_template) = helper.loader_template {
                    cocos_codegen.loader_template = loader_template;
                }
                
                // 处理存储配置
                let storage_path = helper.storage.unwrap_or("assets/table/data".to_string());
                let storage_type = helper.storage_type.unwrap_or("json".to_string());
                
                if storage_type == "json" {
                    cocos_codegen.storage = CocosStorage::Json(CocosJsonConfig {
                        enable: true,
                        output: storage_path.clone(),
                    });
                }
                
                // 处理调试存储配置
                if let Some(storage_debug_type) = helper.storage_debug_type {
                    if storage_debug_type == "json" {
                        cocos_codegen.storage_debug = Some(CocosStorage::Json(CocosJsonConfig {
                            enable: true,
                            output: storage_path,
                        }));
                    }
                }
                
                Ok(Generator::Cocos(cocos_codegen))
            }
            "typescript" => {
                let mut typescript_codegen = TypeScriptCodegen::default();
                
                // 使用配置文件中的值
                if let Some(enable) = helper.enable {
                    typescript_codegen.enable = enable;
                }
                if let Some(project) = helper.project {
                    typescript_codegen.project = project;
                }
                if let Some(loader) = helper.loader {
                    typescript_codegen.output = loader;
                }
                if let Some(loader_template) = helper.loader_template {
                    typescript_codegen.loader_template = loader_template;
                }
                if let Some(storage) = helper.storage {
                    typescript_codegen.storage = storage;
                }
                if let Some(storage_type) = helper.storage_type {
                    typescript_codegen.storage_type = storage_type;
                }
                
                Ok(Generator::TypeScript(typescript_codegen))
            }
            _ => {
                Ok(Generator::Cocos(CocosCodegen::default()))
            }
        }
    }
}
