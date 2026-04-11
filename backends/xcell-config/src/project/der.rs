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
    #[serde(rename = "type")]
    r#type: String,
    enable: Option<bool>,
    project: Option<String>,
    output: Option<String>,
    namespace: Option<String>,
    manager: Option<String>,
    suffix_table: Option<String>,
    suffix_element: Option<String>,
    support_clone: Option<bool>,
    legacy_using: Option<bool>,
    legacy_null_null: Option<bool>,
    xlua: Option<UnityXluaConfig>,
    storage: Option<UnityStorage>,
    development: Option<UnityStorage>,
    cocos: Option<CocosHelper>,
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
        println!("Generator output: {:?}", helper.output);
        println!("Generator cocos: {:?}", helper.cocos);

        match type_name.as_str() {
            "unity" => Ok(Generator::Unity(UnityCodegen {
                enable: helper.enable.unwrap_or(true),
                project: helper.project.unwrap_or(".".to_string()),
                output: helper.output
                    .unwrap_or("Assets/Scripts/DataTable/Generated".to_string()),
                namespace: helper.namespace.unwrap_or("DataTable".to_string()),
                manager: helper.manager.unwrap_or("DataTableManager".to_string()),
                suffix_table: helper.suffix_table.unwrap_or("Table".to_string()),
                suffix_element: helper.suffix_element.unwrap_or_default(),
                support_clone: helper.support_clone.unwrap_or(false),
                legacy_using: helper.legacy_using.unwrap_or(false),
                legacy_null_null: helper.legacy_null_null.unwrap_or(false),
                xlua: helper.xlua.unwrap_or_default(),
                storage: helper.storage.unwrap_or_default(),
                storage_debug: helper.development,
            })),
            "cocos" => {
                let mut cocos_codegen = CocosCodegen::default();
                
                // 使用默认配置
                cocos_codegen.enable = helper.enable.unwrap_or(true);
                cocos_codegen.project = helper.project.unwrap_or("..".to_string());
                cocos_codegen.output = helper.output
                    .unwrap_or("assets/scripts/dataTable/generated".to_string());
                
                // 处理 Cocos 特定配置
                if let Some(cocos_helper) = &helper.cocos {
                    // 处理存储配置
                    if let Some(storage) = &cocos_helper.storage {
                        cocos_codegen.storage = storage.clone();
                    }
                    
                    // 处理开发时存储配置
                    if let Some(development) = &cocos_helper.development {
                        cocos_codegen.storage_debug = Some(development.clone());
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
