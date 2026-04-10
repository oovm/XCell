use super::*;
use crate::unity::{UnityStorage, UnityXluaConfig};
use serde::{Deserialize, Deserializer};
use oak_json::JsonValueNode as JsonValue;

#[derive(Deserialize)]
struct GeneratorHelper {
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
}

impl<'de> Deserialize<'de> for Generator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = GeneratorHelper::deserialize(deserializer)?;
        let type_name = helper.r#type.to_ascii_lowercase();

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
            "cocos" => Ok(Generator::Cocos(CocosCodegen {
                enable: helper.enable.unwrap_or(true),
                project: helper.project.unwrap_or("..".to_string()),
                output: helper.output
                    .unwrap_or("assets/scripts/dataTable/generated".to_string()),
                ..Default::default()
            })),
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
