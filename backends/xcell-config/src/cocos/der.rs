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
        
        match helper.r#type.as_str() {
            "json" => Ok(CocosStorage::Json(CocosJsonConfig {
                enable: helper.enable.unwrap_or(true),
                output: helper.output.unwrap_or_default(),
            })),
            _ => Err(serde::de::Error::custom("Invalid Cocos storage type")),
        }
    }
}

impl<'de> Deserialize<'de> for CocosCodegen {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CocosCodegenHelper {
            storage: CocosStorage,
            development: Option<CocosStorage>,
            enable: bool,
            project: String,
            output: String,
            manager_name: String,
            suffix_table: String,
            instance_name: String,
        }

        let helper = CocosCodegenHelper::deserialize(deserializer)?;
        
        Ok(CocosCodegen {
            storage: helper.storage,
            development: helper.development,
            enable: helper.enable,
            project: helper.project,
            output: helper.output,
            manager_name: helper.manager_name,
            suffix_table: helper.suffix_table,
            instance_name: helper.instance_name,
        })
    }
}