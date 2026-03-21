// 序列化实现
use super::*;
use serde::{Serializer, Serialize};

impl Serialize for UnityStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UnityStorage::Binary(config) => config.serialize(serializer),
            UnityStorage::Json(config) => config.serialize(serializer),
            UnityStorage::Xml(config) => config.serialize(serializer),
            UnityStorage::Protobuf(config) => config.serialize(serializer),
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
            compile_storage: Option<UnityStorage>,
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
            xlua: UnityXluaConfig,
        }

        let helper = UnityCodegenHelper {
            storage: self.storage.clone(),
            compile_storage: self.compile_storage.clone(),
            runtime_storage: self.runtime_storage.clone(),
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
