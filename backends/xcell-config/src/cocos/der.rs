// 反序列化实现
use super::*;
use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for CocosCodegen {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CocosCodegenHelper {
            #[serde(default)]
            storage: CocosStorage,
            enable: bool,
            project: String,
            output: String,
            namespace: String,
            manager_name: String,
            suffix_table: String,
            instance_name: String,
        }

        let helper = CocosCodegenHelper::deserialize(deserializer)?;
        
        Ok(CocosCodegen {
            storage: helper.storage,
            enable: helper.enable,
            project: helper.project,
            output: helper.output,
            namespace: helper.namespace,
            manager_name: helper.manager_name,
            suffix_table: helper.suffix_table,
            instance_name: helper.instance_name,
        })
    }
}
