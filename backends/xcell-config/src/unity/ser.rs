// 序列化实现
use super::*;
use serde::{Serializer, Serialize};
use serde::ser::SerializeStruct;

impl Serialize for UnityStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UnityStorage::Binary(config) => {
                let mut state = serializer.serialize_struct("UnityStorage", 3)?;
                state.serialize_field("type", "binary")?;
                state.serialize_field("enable", &config.enable)?;
                state.serialize_field("output", &config.output)?;
                state.end()
            },
            UnityStorage::Json(config) => {
                let mut state = serializer.serialize_struct("UnityStorage", 3)?;
                state.serialize_field("type", "json")?;
                state.serialize_field("enable", &config.enable)?;
                state.serialize_field("output", &config.output)?;
                state.end()
            },
            UnityStorage::Xml(config) => {
                let mut state = serializer.serialize_struct("UnityStorage", 3)?;
                state.serialize_field("type", "xml")?;
                state.serialize_field("enable", &config.enable)?;
                state.serialize_field("output", &config.output)?;
                state.end()
            },
            UnityStorage::Protobuf(config) => {
                let mut state = serializer.serialize_struct("UnityStorage", 2)?;
                state.serialize_field("type", "protobuf")?;
                state.serialize_field("enable", &config.enable)?;
                state.end()
            },
        }
    }
}

impl Serialize for UnityCodegen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct UnityCodegenHelper {
            storage: UnityStorage,
            development: Option<UnityStorage>,
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
            xlua: UnityXluaConfig,
        }

        let helper = UnityCodegenHelper {
            storage: self.storage.clone(),
            development: self.storage_debug.clone(),
            enable: self.enable,
            project: self.project.clone(),
            output: self.output.clone(),
            namespace: self.namespace.clone(),
            manager: self.manager.clone(),
            suffix_table: self.suffix_table.clone(),
            suffix_element: self.suffix_element.clone(),
            support_clone: self.support_clone,
            legacy_using: self.legacy_using,
            legacy_null_null: self.legacy_null_null,
            xlua: self.xlua.clone(),
        };

        helper.serialize(serializer)
    }
}