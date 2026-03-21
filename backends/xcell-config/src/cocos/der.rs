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
            #[serde(default)]
            loader: CocosLoader,
            // 旧格式字段
            enable: Option<bool>,
            project: Option<String>,
            output: Option<String>,
            namespace: Option<String>,
            manager_name: Option<String>,
            suffix_table: Option<String>,
            instance_name: Option<String>,
            #[serde(default)]
            json: CocosJsonConfig,
        }

        let helper = CocosCodegenHelper::deserialize(deserializer)?;
        
        // 检查是否使用旧格式
        if helper.enable.is_some() || helper.project.is_some() || helper.output.is_some() {
            // 旧格式
            Ok(CocosCodegen {
                storage: CocosStorage {
                    json: helper.json,
                },
                loader: CocosLoader {
                    enable: helper.enable.unwrap_or(false),
                    project: helper.project.unwrap_or_default(),
                    output: helper.output.unwrap_or_default(),
                    namespace: helper.namespace.unwrap_or_default(),
                    manager_name: helper.manager_name.unwrap_or_default(),
                    suffix_table: helper.suffix_table.unwrap_or_default(),
                    instance_name: helper.instance_name.unwrap_or_default(),
                },
            })
        } else {
            // 新格式
            Ok(CocosCodegen {
                storage: helper.storage,
                loader: helper.loader,
            })
        }
    }
}
