// 反序列化实现
use super::*;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
struct CocosStorageHelper {
    r#type: String,
    enable: Option<bool>,
    output: Option<String>,
}

impl<'de> Deserialize<'de> for CocosStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = CocosStorageHelper::deserialize(deserializer)?;

        match helper.r#type.to_ascii_lowercase().as_str() {
            "json" => Ok(CocosStorage::Json(CocosJsonConfig {
                enable: helper.enable.unwrap_or(true),
                output: helper.output.unwrap_or_default(),
            })),
            _ => Err(serde::de::Error::custom("Invalid Cocos storage type")),
        }
    }
}

#[derive(Deserialize)]
struct CocosLoaderHelper {
    enable: Option<bool>,
    project: Option<String>,
    output: Option<String>,
    namespace: Option<String>,
    manager_name: Option<String>,
    suffix_table: Option<String>,
    instance_name: Option<String>,
}

#[derive(Deserialize)]
struct CocosCodegenHelper {
    storage: Option<CocosStorage>,
    development: Option<CocosStorage>,
    loader: Option<CocosLoaderHelper>,
}

impl<'de> Deserialize<'de> for CocosCodegen {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = CocosCodegenHelper::deserialize(deserializer)?;
        
        // 从 loader 子表中读取配置
        let loader = helper.loader.unwrap_or_default();

        Ok(CocosCodegen {
            storage: helper.storage.unwrap_or_default(),
            storage_debug: helper.development,
            enable: loader.enable.unwrap_or(true),
            project: loader.project.unwrap_or("../".to_string()),
            output: loader.output.unwrap_or("assets/scripts/dataTable/generated".to_string()),
            manager_name: loader.manager_name.unwrap_or("DataTableManager".to_string()),
            suffix_table: loader.suffix_table.unwrap_or("Table".to_string()),
            instance_name: loader.instance_name.unwrap_or("DataTable".to_string()),
        })
    }
}
