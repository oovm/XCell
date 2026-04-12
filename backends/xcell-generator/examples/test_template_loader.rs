use xcell_generator::template::{TemplateLoader, TemplateType};
use nargo_types::NargoValue;
use std::collections::HashMap;

fn main() {
    eprintln!("=== TemplateLoader 渲染测试 ===\n");

    let loader = TemplateLoader::new(None).unwrap();

    // 测试 BuildClass 模板渲染
    let mut context_data = HashMap::new();
    context_data.insert("compiler_version".to_string(), NargoValue::String("0.2.0".to_string()));
    context_data.insert("class_name".to_string(), NargoValue::String("Attribute".to_string()));
    context_data.insert("table_name".to_string(), NargoValue::String("AttributeTable".to_string()));
    context_data.insert("key_name".to_string(), NargoValue::String("key".to_string()));
    context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));

    let class_fields: Vec<NargoValue> = vec![
        {
            let mut f = HashMap::new();
            f.insert("document".to_string(), NargoValue::Array(vec![]));
            f.insert("name".to_string(), NargoValue::String("id".to_string()));
            f.insert("typing".to_string(), NargoValue::String("number".to_string()));
            f.insert("has_default".to_string(), NargoValue::Bool(false));
            f.insert("default".to_string(), NargoValue::String(String::new()));
            NargoValue::Object(f)
        },
        {
            let mut f = HashMap::new();
            f.insert("document".to_string(), NargoValue::Array(vec![]));
            f.insert("name".to_string(), NargoValue::String("key".to_string()));
            f.insert("typing".to_string(), NargoValue::String("string".to_string()));
            f.insert("has_default".to_string(), NargoValue::Bool(false));
            f.insert("default".to_string(), NargoValue::String(String::new()));
            NargoValue::Object(f)
        },
    ];
    context_data.insert("class_fields".to_string(), NargoValue::Array(class_fields));

    let context = NargoValue::Object(context_data);

    match loader.render_with_dejavu(TemplateType::Class.file_name(), &context) {
        Ok(result) => {
            eprintln!("渲染成功!");
            println!("{}", result);
            if result.contains("<%") {
                eprintln!("\n警告: 渲染结果包含原始 DejaVu 语法!");
            }
        },
        Err(e) => {
            eprintln!("渲染失败: {:?}", e);
        }
    }
}
