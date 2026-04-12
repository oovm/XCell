use nargo_template::{DejaVuFrontend, Frontend, TemplateIR, UnifiedTemplateEngine};
use nargo_types::NargoValue;
use std::collections::HashMap;

fn main() {
    eprintln!("=== DejaVu 模板渲染调试 ===\n");

    let frontend = DejaVuFrontend::new();

    // 测试 BuildEnumerate 模板
    let template = r#"// 代码生成, 修改无效! (XCell <% compiler_version %>)

<% if namespace %>
export namespace <% namespace %> {
<% end if %>

export enum <% class_name %> {
<% loop field in enumerate_ids %>
    <% loop line in field.document %>
    /** <% line %> */
    <% end loop %>
    <% field.key %> = <% field.value %>,
<% end loop %>
}

<% if namespace %>
}
<% end if %>"#;

    eprintln!("--- 编译 BuildEnumerate 模板 ---");
    match frontend.compile(template) {
        Ok(ir) => {
            eprintln!("编译成功! IR 指令数: {}", ir.instructions.len());
            for (i, instr) in ir.instructions.iter().enumerate() {
                eprintln!("  {:3}: {:?}", i, instr);
            }
        },
        Err(e) => {
            eprintln!("编译失败: {}", e);
        }
    }

    // 测试完整渲染
    let mut adapter = nargo_template::DejaVuAdapter::new(DejaVuFrontend::new());
    match adapter.register_template("enumerate", template) {
        Ok(_) => eprintln!("\n注册模板成功"),
        Err(e) => eprintln!("\n注册模板失败: {}", e),
    }

    let mut ctx = HashMap::new();
    ctx.insert("compiler_version".to_string(), NargoValue::String("0.1.0".to_string()));
    ctx.insert("namespace".to_string(), NargoValue::String("Game".to_string()));
    ctx.insert("class_name".to_string(), NargoValue::String("Quality".to_string()));
    let mut ids = Vec::new();
    for (k, v, doc) in [("Common", "0", "普通品质"), ("Uncommon", "1", "非凡品质"), ("Rare", "2", "稀有品质")] {
        let mut item = HashMap::new();
        item.insert("key".to_string(), NargoValue::String(k.to_string()));
        item.insert("value".to_string(), NargoValue::String(v.to_string()));
        item.insert("document".to_string(), NargoValue::Array(vec![NargoValue::String(doc.to_string())]));
        ids.push(NargoValue::Object(item));
    }
    ctx.insert("enumerate_ids".to_string(), NargoValue::Array(ids));

    match adapter.render("enumerate", &NargoValue::Object(ctx)) {
        Ok(result) => {
            eprintln!("\n渲染结果:");
            println!("{}", result);
        },
        Err(e) => eprintln!("\n渲染失败: {}", e),
    }
}
