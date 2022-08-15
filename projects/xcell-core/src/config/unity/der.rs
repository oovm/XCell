use std::any::type_name;

use serde::de::{Error, MapAccess, Visitor};

use super::*;

default_deserialize![UnityCodegen, UnityBinaryConfig, UnityXmlConfig];

impl Default for UnityCodegen {
    fn default() -> Self {
        Self {
            enable: false,
            version: "1.0.0".to_string(),
            project: "../".to_string(),
            output: "Assets/Scripts/DataTable/Generated".to_string(),
            namespace: "DataTable.Generated".to_string(),
            instance_name: "instance".to_string(),
            manager_name: "DataTableManager".to_string(),
            suffix_table: "Table".to_string(),
            binary: Default::default(),
            support_clone: true,
            legacy_using: false,
            legacy_null_null: false,
            xml: UnityXmlConfig { enable: false, output: "".to_string() },
        }
    }
}

impl Default for UnityBinaryConfig {
    fn default() -> Self {
        Self { enable: true, output: "Assets/Tables/Binary".to_string() }
    }
}

impl<'de> Visitor<'de> for UnityCodegen {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(type_name::<Self>())
    }

    fn visit_bool<E>(mut self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.enable = v;
        Ok(self)
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "enable" => read_map_next_value(&mut map, |v| self.enable = v),
                "version" => read_map_next_value(&mut map, |v| self.version = v),
                "project" => read_map_next_value(&mut map, |v| self.project = v),
                "output" => read_map_next_value(&mut map, |v| self.output = v),
                "manager_name" | "manager" => read_map_next_value(&mut map, |v| self.manager_name = v),
                "instance_name" | "instance" => read_map_next_value(&mut map, |v| self.instance_name = v),
                "namespace" => read_map_next_value(&mut map, |v| self.namespace = v),
                //
                "table_suffix" | "suffix_table" | "table" => read_map_next_value(&mut map, |v| self.suffix_table = v),
                "support_clone" => read_map_next_value(&mut map, |v| self.support_clone = v),
                // legacy
                "legacy" => read_map_next_value(&mut map, |v| {
                    self.legacy_using = v;
                    self.legacy_null_null = v;
                }),
                "legacy_using" => read_map_next_value(&mut map, |v| self.legacy_using = v),
                "legacy_null_null" => read_map_next_value(&mut map, |v| self.legacy_null_null = v),
                "binary" => read_map_next_value(&mut map, |v| self.binary = v),
                "xml" => read_map_next_value(&mut map, |v| self.xml = v),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}

impl<'de> Visitor<'de> for UnityBinaryConfig {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(type_name::<Self>())
    }

    fn visit_bool<E>(mut self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.enable = v;
        Ok(self)
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "enable" => read_map_next_value(&mut map, |v| self.enable = v),
                "output" => read_map_next_value(&mut map, |v| self.output = v),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}

impl Default for UnityXmlConfig {
    fn default() -> Self {
        Self { enable: false, output: "Assets/Tables/Readable".to_string() }
    }
}

impl<'de> Visitor<'de> for UnityXmlConfig {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(type_name::<Self>())
    }

    fn visit_bool<E>(mut self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.enable = v;
        Ok(self)
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "enable" => read_map_next_value(&mut map, |v| self.enable = v),
                "output" => read_map_next_value(&mut map, |v| self.output = v),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}
