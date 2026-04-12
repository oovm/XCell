use crate::{XResult, WorkspaceManager, XClassData, XDictData, XEnumerateData, XListData};
use serde::Serialize;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use url::Url;
use xcell_core::XCellValue;
use oak_json::language::JsonValue;
use oak_json::language::value::{JsonArray, JsonObject};
use crate::codegen::core::typescript::AsTypeScriptType;

/// JSON 代码生成器配置
#[derive(Clone, Debug, Serialize)]
pub struct JsonCodegen {
    /// 是否启用 JSON 生成
    pub enable: bool,
    /// 输出目录
    pub output: String,
    /// 是否格式化输出
    pub pretty: bool,
    /// 是否允许尾随逗号
    pub trailing_comma: bool,
}

impl Default for JsonCodegen {
    fn default() -> Self {
        JsonCodegen {
            enable: false,
            output: "json".to_string(),
            pretty: true,
            trailing_comma: false,
        }
    }
}

impl JsonCodegen {
    /// 获取 JSON 文件输出路径
    pub fn json_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = root.join(&self.output);
        let path = dir.join(file_name).with_extension("json");
        Ok(path)
    }

    /// 获取 JSON 相对路径
    pub fn json_relative(&self, file_name: &str) -> String {
        format!("{}/{}.json", self.output, file_name)
    }

    /// 确保输出目录存在
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if self.enable {
            if let Some(s) = self.json_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
        }
        Ok(())
    }

    /// 写入 JSON 数据
    pub fn write_json(&self, ws: &WorkspaceManager) -> XResult<()> {
        if !self.enable {
            return Ok(());
        }

        self.ensure_path(&ws.config.root)?;

        for table in ws.classes() {
            if let Err(e) = self.write_class(ws, table) {
                tracing::error!("生成JSON类失败: {}", e);
            }
        }
        for table in ws.enumerates() {
            if let Err(e) = self.write_enumerate(ws, table) {
                tracing::error!("生成JSON枚举失败: {}", e);
            }
        }
        for table in ws.dicts() {
            if let Err(e) = self.write_dict(ws, table) {
                tracing::error!("生成JSON字典失败: {}", e);
            }
        }
        for table in ws.lists() {
            if let Err(e) = self.write_list(ws, table) {
                tracing::error!("生成JSON列表失败: {}", e);
            }
        }

        Ok(())
    }

    /// 写入类表 JSON
    fn write_class(&self, ws: &WorkspaceManager, table: &XClassData) -> XResult<()> {
        let json_data = self.make_class(table);
        self.write_json_file(ws, &table.name, &json_data)
    }

    /// 写入枚举表 JSON
    fn write_enumerate(&self, ws: &WorkspaceManager, table: &XEnumerateData) -> XResult<()> {
        let json_data = self.make_enumerate(table);
        self.write_json_file(ws, &table.name, &json_data)
    }

    /// 写入字典表 JSON
    fn write_dict(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
        let json_data = self.make_dict(table);
        self.write_json_file(ws, &table.name, &json_data)
    }

    /// 写入列表表 JSON
    fn write_list(&self, ws: &WorkspaceManager, table: &XListData) -> XResult<()> {
        let json_data = self.make_list(table);
        self.write_json_file(ws, &table.name, &json_data)
    }

    /// 使用 oak-json 写入 JSON 文件
    fn write_json_file(&self, ws: &WorkspaceManager, name: &str, data: &impl Serialize) -> XResult<()> {
        let path = self.json_path(&ws.config.root, name)?;
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json_string = oak_json::to_string(data)
            .map_err(|e| xcell_core::XError::runtime_error(format!("JSON序列化失败: {:?}", e)))?;

        let mut file = File::create(&path)?;
        file.write_all(json_string.as_bytes())?;

        tracing::info!("写入 JSON: {}\n{}", self.json_relative(name), Url::from_file_path(&path)?);
        Ok(())
    }

    /// 生成类表 JSON 数据
    fn make_class(&self, table: &XClassData) -> JsonClassData {
        JsonClassData {
            name: table.name.clone(),
            fields: table.items.iter().map(|item| JsonFieldData {
                name: item.field.clone(),
                r#type: item.typing.as_typescript_type().to_string(),
                default: self.xcell_value_to_json(&item.default),
                comment: item.document.lines(),
            }).collect(),
        }
    }

    /// 生成枚举表 JSON 数据
    fn make_enumerate(&self, table: &XEnumerateData) -> JsonEnumerateData {
        JsonEnumerateData {
            name: table.name.clone(),
            id_type: table.typing.kind.as_typescript_type().to_string(),
            values: table.lines.iter().map(|line| JsonEnumValue {
                id: line.id.to_string(),
                key: line.key.clone(),
                comment: line.comment.lines(),
                fields: table.headers.iter().enumerate().map(|(i, header)| {
                    let value = line.data.get(i);
                    JsonFieldData {
                        name: header.field_name.clone(),
                        r#type: header.typing.as_typescript_type().to_string(),
                        default: value.map(|v| self.xcell_value_to_json(v)).unwrap_or(JsonValue::Null),
                        comment: header.document.lines(),
                    }
                }).collect(),
            }).collect(),
        }
    }

    /// 生成字典表 JSON 数据
    fn make_dict(&self, table: &XDictData) -> JsonDictData {
        JsonDictData {
            name: table.name.clone(),
            entries: table.mapping.iter().map(|(key, line)| {
                JsonDictEntry {
                    key: key.clone(),
                    fields: table.headers.iter().enumerate().map(|(i, header)| {
                        let value = line.data.get(i);
                        (header.field_name.clone(), value.map(|v| self.xcell_value_to_json(v)).unwrap_or(JsonValue::Null))
                    }).collect(),
                }
            }).collect(),
        }
    }

    /// 生成列表表 JSON 数据
    fn make_list(&self, table: &XListData) -> JsonListData {
        JsonListData {
            name: table.name.clone(),
            id_type: table.id_type.as_typescript_type().to_string(),
            entries: table.mapping.iter().map(|(id, line)| {
                JsonListEntry {
                    id: id.to_string(),
                    key: line.key.clone(),
                    fields: table.headers.iter().enumerate().map(|(i, header)| {
                        let value = line.data.get(i);
                        (header.field_name.clone(), value.map(|v| self.xcell_value_to_json(v)).unwrap_or(JsonValue::Null))
                    }).collect(),
                }
            }).collect(),
        }
    }

    /// 将 XCellValue 转换为 JsonValue
    fn xcell_value_to_json(&self, value: &XCellValue) -> JsonValue {
        match value {
            XCellValue::Boolean(b) => JsonValue::Boolean(*b),
            XCellValue::Integer8(i) => JsonValue::Integer(*i as i64),
            XCellValue::Integer16(i) => JsonValue::Integer(*i as i64),
            XCellValue::Integer32(i) => JsonValue::Integer(*i as i64),
            XCellValue::Integer64(i) => JsonValue::Integer(*i),
            XCellValue::Unsigned8(u) => JsonValue::Integer(*u as i64),
            XCellValue::Unsigned16(u) => JsonValue::Integer(*u as i64),
            XCellValue::Unsigned32(u) => JsonValue::Integer(*u as i64),
            XCellValue::Unsigned64(u) => JsonValue::Integer(*u as i64),
            XCellValue::Float32(f) => JsonValue::Float(*f as f64),
            XCellValue::Float64(f) => JsonValue::Float(*f),
            XCellValue::String(s) => JsonValue::String(s.clone()),
            XCellValue::Vector2(v) => JsonValue::Array(JsonArray {
                list: vec![
                    JsonValue::Float(v[0] as f64),
                    JsonValue::Float(v[1] as f64)
                ]
            }),
            XCellValue::Vector3(v) => JsonValue::Array(JsonArray {
                list: vec![
                    JsonValue::Float(v[0] as f64),
                    JsonValue::Float(v[1] as f64),
                    JsonValue::Float(v[2] as f64)
                ]
            }),
            XCellValue::Vector4(v) => JsonValue::Array(JsonArray {
                list: vec![
                    JsonValue::Float(v[0] as f64),
                    JsonValue::Float(v[1] as f64),
                    JsonValue::Float(v[2] as f64),
                    JsonValue::Float(v[3] as f64)
                ]
            }),
            XCellValue::Quaternion4(v) => JsonValue::Array(JsonArray {
                list: vec![
                    JsonValue::Float(v[0] as f64),
                    JsonValue::Float(v[1] as f64),
                    JsonValue::Float(v[2] as f64),
                    JsonValue::Float(v[3] as f64)
                ]
            }),
            XCellValue::Color(c) => JsonValue::Object(JsonObject {
                dict: std::collections::HashMap::from([
                    ("r".to_string(), JsonValue::Float(c.r as f64)),
                    ("g".to_string(), JsonValue::Float(c.g as f64)),
                    ("b".to_string(), JsonValue::Float(c.b as f64)),
                    ("a".to_string(), JsonValue::Float(c.a as f64))
                ])
            }),
            XCellValue::Vector(v) => JsonValue::Array(JsonArray {
                list: v.iter().map(|item| self.xcell_value_to_json(item)).collect()
            }),
            XCellValue::Enumerate(s) => JsonValue::String(s.clone()),
            XCellValue::Reference(r) => JsonValue::Integer(*r as i64),
            XCellValue::Map(m) => JsonValue::Object(JsonObject {
                dict: m.iter().map(|(k, v)| (k.clone(), self.xcell_value_to_json(v))).collect()
            }),
            XCellValue::Optional(o) => match o {
                Some(v) => self.xcell_value_to_json(v),
                None => JsonValue::Null,
            },
        }
    }
}

impl super::Codegen for JsonCodegen {
    fn generate(&self, context: &super::CodegenContext) -> XResult<()> {
        if let Some(workspace) = &context.workspace {
            self.write_json(workspace)?;
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "json"
    }
}

/// 类表 JSON 数据结构
#[derive(Serialize)]
struct JsonClassData {
    name: String,
    fields: Vec<JsonFieldData>,
}

/// 字段 JSON 数据结构
#[derive(Serialize)]
struct JsonFieldData {
    name: String,
    r#type: String,
    default: JsonValue,
    comment: Vec<String>,
}

/// 枚举表 JSON 数据结构
#[derive(Serialize)]
struct JsonEnumerateData {
    name: String,
    id_type: String,
    values: Vec<JsonEnumValue>,
}

/// 枚举值 JSON 数据结构
#[derive(Serialize)]
struct JsonEnumValue {
    id: String,
    key: String,
    comment: Vec<String>,
    fields: Vec<JsonFieldData>,
}

/// 字典表 JSON 数据结构
#[derive(Serialize)]
struct JsonDictData {
    name: String,
    entries: Vec<JsonDictEntry>,
}

/// 字典条目 JSON 数据结构
#[derive(Serialize)]
struct JsonDictEntry {
    key: String,
    fields: std::collections::BTreeMap<String, JsonValue>,
}

/// 列表表 JSON 数据结构
#[derive(Serialize)]
struct JsonListData {
    name: String,
    id_type: String,
    entries: Vec<JsonListEntry>,
}

/// 列表条目 JSON 数据结构
#[derive(Serialize)]
struct JsonListEntry {
    id: String,
    key: String,
    fields: std::collections::BTreeMap<String, JsonValue>,
}
