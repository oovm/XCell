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
        let helper = GeneratorHelper::deserialize(deserializer)?;
        let type_name = helper.r#type.to_ascii_lowercase();

        println!("Generator type: {}", type_name);
        println!("Generator enable: {:?}", helper.enable);
        println!("Generator project: {:?}", helper.project);
        println!("Generator loader: {:?}", helper.loader);
        println!("Generator storage: {:?}", helper.storage);
        println!("Generator storage_type: {:?}", helper.storage_type);
        println!("Generator storage_debug_type: {:?}", helper.storage_debug_type);

        match type_name.as_str() {
            "unity" => Ok(Generator::Unity(UnityCodegen::default())),
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
            "xlua" => Ok(Generator::Xlua(XluaCodegen::default())),
            "sql" => Ok(Generator::Sql(SqlCodegen::default())),
            "json" => Ok(Generator::Json(JsonCodegen::default())),
            "typescript" => Ok(Generator::TypeScript(TypeScriptCodegen::default())),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid generator type: {}",
                type_name
            ))),
        }
    }
}
