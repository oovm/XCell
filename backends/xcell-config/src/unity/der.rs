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
            "protobuf" => Ok(UnityStorage::Protobuf(UnityProtobufConfig { enable: helper.enable.unwrap_or(true) })),
            _ => Err(serde::de::Error::custom("Invalid Unity storage type")),
        }
    }
}

#[derive(Deserialize)]
struct UnityCodegenHelper {
    storage: UnityStorage,
    development: Option<UnityStorage>,
    enable: Option<bool>,
    project: String,
    output: String,
    namespace: String,
    manager: String,
    suffix_table: String,
    suffix_element: String,
    support_clone: bool,
    legacy_using: bool,
    legacy_null_null: bool,
    xlua: UnityXluaConfig,
}

impl<'de> Deserialize<'de> for UnityCodegen {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = UnityCodegenHelper::deserialize(deserializer)?;

        Ok(UnityCodegen {
            enable: helper.enable.unwrap_or(true),
            storage: helper.storage,
            storage_debug: helper.development,
            project: helper.project,
            output: helper.output,
            namespace: helper.namespace,
            manager: helper.manager,
            suffix_table: helper.suffix_table,
            suffix_element: helper.suffix_element,
            support_clone: helper.support_clone,
            legacy_using: helper.legacy_using,
            legacy_null_null: helper.legacy_null_null,
            xlua: helper.xlua,
        })
    }
}
