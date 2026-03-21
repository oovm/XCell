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
            compile_storage: Option<UnityStorage>,
            #[serde(default)]
            runtime_storage: Option<UnityStorage>,
            enable: bool,
            project: String,
            output: String,
            namespace: String,
            manager: String,
            suffix_table: String,
            suffix_element: String,
            support_clone: bool,
            legacy_using: bool,
            legacy_null_null: bool,
            #[serde(default)]
            xlua: UnityXluaConfig,
        }

        let helper = UnityCodegenHelper::deserialize(deserializer)?;
        
        Ok(UnityCodegen {
            storage: helper.storage,
            compile_storage: helper.compile_storage,
            runtime_storage: helper.runtime_storage,
            enable: helper.enable,
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
