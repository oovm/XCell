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
            "protobuf" => Ok(UnityStorage::Protobuf(UnityProtobufConfig {
                enable: helper.enable.unwrap_or(true),
                output: helper.output.unwrap_or_default(),
            })),
            _ => Err(serde::de::Error::custom("Invalid Unity storage type")),
        }
    }
}

#[derive(Deserialize, Default)]
struct UnityFlatHelper {
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
}

#[derive(Deserialize)]
struct UnityCodegenHelper {
    #[serde(default)]
    loader: Option<UnityFlatHelper>,
    #[serde(default)]
    enable: Option<bool>,
    #[serde(default)]
    project: Option<String>,
    #[serde(default)]
    output: Option<String>,
    #[serde(default)]
    namespace: Option<String>,
    #[serde(default)]
    manager: Option<String>,
    #[serde(default)]
    suffix_table: Option<String>,
    #[serde(default)]
    suffix_element: Option<String>,
    #[serde(default)]
    support_clone: Option<bool>,
    #[serde(default)]
    legacy_using: Option<bool>,
    #[serde(default)]
    legacy_null_null: Option<bool>,
    #[serde(default)]
    xlua: Option<UnityXluaConfig>,
    #[serde(default)]
    storage: Option<UnityStorage>,
    #[serde(default)]
    development: Option<UnityStorage>,
}

impl<'de> Deserialize<'de> for UnityCodegen {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = UnityCodegenHelper::deserialize(deserializer)?;

        if let Some(loader) = helper.loader {
            return Ok(UnityCodegen {
                enable: loader.enable.unwrap_or(true),
                storage: loader.storage.unwrap_or_default(),
                storage_debug: loader.development,
                project: loader.project.unwrap_or(".".to_string()),
                output: loader.output
                    .unwrap_or("Assets/Scripts/DataTable/Generated".to_string()),
                namespace: loader.namespace.unwrap_or("DataTable".to_string()),
                manager: loader.manager.unwrap_or("DataTableManager".to_string()),
                suffix_table: loader.suffix_table.unwrap_or("Table".to_string()),
                suffix_element: loader.suffix_element.unwrap_or_default(),
                support_clone: loader.support_clone.unwrap_or(false),
                legacy_using: loader.legacy_using.unwrap_or(false),
                legacy_null_null: loader.legacy_null_null.unwrap_or(false),
                xlua: loader.xlua.unwrap_or_default(),
            });
        }

        Ok(UnityCodegen {
            enable: helper.enable.unwrap_or(true),
            storage: helper.storage.unwrap_or_default(),
            storage_debug: helper.development,
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
        })
    }
}
