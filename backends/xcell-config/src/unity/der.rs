// 反序列化实现
use super::*;
use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for UnityCodegen {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct UnityCodegenHelper {
            #[serde(default)]
            storage: UnityStorage,
            #[serde(default)]
            loader: UnityLoader,
            // 旧格式字段
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
            #[serde(default)]
            binary: UnityBinaryConfig,
            #[serde(default)]
            xlua: UnityXluaConfig,
            #[serde(default)]
            xml: UnityXmlConfig,
            #[serde(default)]
            json: UnityJsonConfig,
            #[serde(default)]
            protobuf: UnityProtobufConfig,
        }

        let helper = UnityCodegenHelper::deserialize(deserializer)?;
        
        // 检查是否使用旧格式
        if helper.enable.is_some() || helper.project.is_some() || helper.output.is_some() {
            // 旧格式
            Ok(UnityCodegen {
                storage: UnityStorage {
                    binary: helper.binary,
                    json: helper.json,
                    xml: helper.xml,
                    protobuf: helper.protobuf,
                },
                loader: UnityLoader {
                    enable: helper.enable.unwrap_or(false),
                    project: helper.project.unwrap_or_default(),
                    output: helper.output.unwrap_or_default(),
                    namespace: helper.namespace.unwrap_or_default(),
                    manager: helper.manager.unwrap_or_default(),
                    suffix_table: helper.suffix_table.unwrap_or_default(),
                    suffix_element: helper.suffix_element.unwrap_or_default(),
                    support_clone: helper.support_clone.unwrap_or(false),
                    legacy_using: helper.legacy_using.unwrap_or(false),
                    legacy_null_null: helper.legacy_null_null.unwrap_or(false),
                    xlua: helper.xlua,
                },
            })
        } else {
            // 新格式
            Ok(UnityCodegen {
                storage: helper.storage,
                loader: helper.loader,
            })
        }
    }
}
