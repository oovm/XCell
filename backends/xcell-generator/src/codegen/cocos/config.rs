// Cocos 代码生成配置

/// 类型映射配置
pub const TYPE_MAPPINGS: &'static [(&'static str, &'static str)] = &[
    ("i32", "number"),
    ("i64", "number"),
    ("u32", "number"),
    ("u64", "number"),
    ("f32", "number"),
    ("f64", "number"),
    ("text", "string"),
    ("string", "string"),
    ("any", "string"),
];

/// 特殊字段名配置
pub const SPECIAL_FIELDS: &'static [&'static str] = &[
    "type",
    "level",
    "level_requirement",
    "drop_items",
    "skills",
    "unlock_skills",
];

/// 特殊类名配置
pub const SPECIAL_CLASSES: &'static [&'static str] = &[
    "Monster",
    "Skill",
];

/// 表头字段配置
pub const HEADER_FIELDS: &'static [&'static str] = &[
    "id",
    "name",
    "description",
    "desc",
];

/// 获取类型映射
pub fn get_type_mapping(rust_type: &str) -> &'static str {
    for (rust, ts) in TYPE_MAPPINGS {
        if rust == &rust_type {
            return ts;
        }
    }
    "string"
}

/// 检查是否是特殊字段
pub fn is_special_field(field_name: &str) -> bool {
    SPECIAL_FIELDS.contains(&field_name)
}

/// 检查是否是特殊类名
pub fn is_special_class(class_name: &str) -> bool {
    SPECIAL_CLASSES.contains(&class_name)
}

/// 检查是否是ID字段
pub fn is_id_field(field_name: &str) -> bool {
    field_name == "id"
}

/// 检查是否是名称字段
pub fn is_name_field(field_name: &str) -> bool {
    field_name == "name"
}

/// 检查是否是描述字段
pub fn is_description_field(field_name: &str) -> bool {
    field_name == "description" || field_name == "desc"
}
