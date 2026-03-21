// 序列化实现
use super::*;
use serde::{Serializer, Serialize};
use serde::ser::SerializeStruct;

impl Serialize for CocosStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CocosStorage::Json(config) => {
                let mut state = serializer.serialize_struct("CocosStorage", 3)?;
                state.serialize_field("type", "json")?;
                state.serialize_field("enable", &config.enable)?;
                state.serialize_field("output", &config.output)?;
                state.end()
            },
        }
    }
}

impl Serialize for CocosCodegen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
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

        let helper = CocosCodegenHelper {
            storage: self.storage.clone(),
            development: self.development.clone(),
            enable: self.enable,
            project: self.project.clone(),
            output: self.output.clone(),
            manager_name: self.manager_name.clone(),
            suffix_table: self.suffix_table.clone(),
            instance_name: self.instance_name.clone(),
        };

        helper.serialize(serializer)
    }
}