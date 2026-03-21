// 反序列化实现
use super::*;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
struct GeneratorHelper {
    r#type: String,
    enable: Option<bool>,
    project: Option<String>,
    output: Option<String>,
    namespace: Option<String>,
    #[serde(flatten)]
    other: serde::de::IgnoredAny,
}

impl<'de> Deserialize<'de> for Generator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = GeneratorHelper::deserialize(deserializer)?;
        let type_name = helper.r#type.to_ascii_lowercase();
        
        match type_name.as_str() {
            "unity" => {
                Ok(Generator::Unity(UnityCodegen {
                    enable: helper.enable.unwrap_or(true),
                    project: helper.project.unwrap_or(".".to_string()),
                    output: helper.output.unwrap_or("Assets/Scripts/DataTable/Generated".to_string()),
                    namespace: helper.namespace.unwrap_or("DataTable".to_string()),
                    ..Default::default()
                }))
            }
            "cocos" => {
                Ok(Generator::Cocos(CocosCodegen {
                    enable: helper.enable.unwrap_or(true),
                    project: helper.project.unwrap_or("..".to_string()),
                    output: helper.output.unwrap_or("assets/scripts/dataTable/generated".to_string()),
                    ..Default::default()
                }))
            }
            "xlua" => {
                Ok(Generator::Xlua(XluaCodegen::default()))
            }
            "sql" => {
                Ok(Generator::Sql(SqlCodegen::default()))
            }
            "json" => {
                Ok(Generator::Json(JsonCodegen::default()))
            }
            "typescript" => {
                Ok(Generator::TypeScript(TypeScriptCodegen::default()))
            }
            _ => Err(serde::de::Error::custom(format!("Invalid generator type: {}", type_name))),
        }
    }
}
