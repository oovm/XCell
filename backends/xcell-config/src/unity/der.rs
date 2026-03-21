// 反序列化实现
use super::*;
use serde::{Deserialize, Deserializer};
#[derive(Deserialize)]
struct UnityStorageHelper {
    r#type: String,
    enable: Option<bool>,
    output: Option<String>,
}

impl<'de> Deserialize<'de> for UnityStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = UnityStorageHelper::deserialize(deserializer)?;
        match helper.r#type.to_ascii_lowercase().as_ref() {
            "binary" => Ok(UnityStorage::Binary(UnityBinaryConfig {
                enable: helper.enable.unwrap_or(true),
                output: helper.output.unwrap_or_default(),
            })),
            "json" => Ok(UnityStorage::Json(UnityJsonConfig {
                enable: helper.enable.unwrap_or(true),
                output: helper.output.unwrap_or_default(),
            })),
            "xml" => Ok(UnityStorage::Xml(UnityXmlConfig {
                enable: helper.enable.unwrap_or(true),
                output: helper.output.unwrap_or_default(),
            })),
            "protobuf" => Ok(UnityStorage::Protobuf(UnityProtobufConfig { enable: helper.enable.unwrap_or(true), output: helper.output.unwrap_or_default() })),
            _ => Err(serde::de::Error::custom("Invalid Unity storage type")),
        }
    }
}

#[derive(Deserialize)]
struct UnityCodegenHelper {
    storage: Option<UnityStorage>,
    development: Option<UnityStorage>,
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
}

impl<'de> Deserialize<'de> for UnityCodegen {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = UnityCodegenHelper::deserialize(deserializer)?;

        Ok(UnityCodegen {
            enable: helper.enable.unwrap_or(true),
            storage: helper.storage.unwrap_or_default(),
            storage_debug: helper.development,
            project: helper.project.unwrap_or("../".to_string()),
            output: helper.output.unwrap_or("Assets/Scripts/DataTable/Generated".to_string()),
            namespace: helper.namespace.unwrap_or("DataTable".to_string()),
            manager: helper.manager.unwrap_or("DataTableManager".to_string()),
            suffix_table: helper.suffix_table.unwrap_or("Table".to_string()),
            suffix_element: helper.suffix_element.unwrap_or("".to_string()),
            support_clone: helper.support_clone.unwrap_or(false),
            legacy_using: helper.legacy_using.unwrap_or(false),
            legacy_null_null: helper.legacy_null_null.unwrap_or(false),
            xlua: helper.xlua.unwrap_or_default(),
        })
    }
}
