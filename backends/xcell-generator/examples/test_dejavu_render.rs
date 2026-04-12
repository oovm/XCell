use nargo_template::{DejaVuFrontend, Frontend, TemplateIR, UnifiedTemplateEngine};
use nargo_types::NargoValue;
use std::collections::HashMap;

fn print_ir(ir: &TemplateIR) {
    for (i, instr) in ir.instructions.iter().enumerate() {
        eprintln!("  {:3}: {:?}", i, instr);
    }
}

fn main() {
    eprintln!("=== DejaVu IR 调试 ===\n");

    let frontend = DejaVuFrontend::new();

    // 测试 1: 简单插值
    eprintln!("--- Test 1: 简单插值 ---");
    let src1 = "Hello <% name %>!";
    match frontend.compile(src1) {
        Ok(ir) => {
            eprintln!("IR:");
            print_ir(&ir);
        },
        Err(e) => eprintln!("编译失败: {}", e),
    }

    // 测试 2: loop 语法
    eprintln!("\n--- Test 2: loop 语法 ---");
    let src2 = "<% loop item in items %><% item %>,<% end loop %>";
    match frontend.compile(src2) {
        Ok(ir) => {
            eprintln!("IR:");
            print_ir(&ir);
        },
        Err(e) => eprintln!("编译失败: {}", e),
    }

    // 测试 3: if 语法
    eprintln!("\n--- Test 3: if 语法 ---");
    let src3 = "<% if show %>visible<% end if %>";
    match frontend.compile(src3) {
        Ok(ir) => {
            eprintln!("IR:");
            print_ir(&ir);
        },
        Err(e) => eprintln!("编译失败: {}", e),
    }

    // 测试 4: field.key 访问
    eprintln!("\n--- Test 4: field.key 访问 ---");
    let src4 = "<% loop field in items %><% field.key %><% end loop %>";
    match frontend.compile(src4) {
        Ok(ir) => {
            eprintln!("IR:");
            print_ir(&ir);
        },
        Err(e) => eprintln!("编译失败: {}", e),
    }

    // 测试 5: if field.has_default
    eprintln!("\n--- Test 5: if field.has_default ---");
    let src5 = "<% loop field in items %><% if field.has_default %>yes<% else %>no<% end if %><% end loop %>";
    match frontend.compile(src5) {
        Ok(ir) => {
            eprintln!("IR:");
            print_ir(&ir);
        },
        Err(e) => eprintln!("编译失败: {}", e),
    }

    // 先解析 AST 看看
    eprintln!("\n--- AST 解析测试 ---");
    let src_if = "<% if show %>visible<% end if %>";
    match oak_dejavu::parse(src_if) {
        Ok(ast) => {
            eprintln!("AST items count: {}", ast.items.len());
            for (i, item) in ast.items.iter().enumerate() {
                eprintln!("  item[{}]: {:?}", i, item);
            }
        },
        Err(e) => eprintln!("AST 解析失败: {}", e),
    }

    let src_loop = "<% loop field in items %><% field.key %><% end loop %>";
    match oak_dejavu::parse(src_loop) {
        Ok(ast) => {
            eprintln!("\nLoop AST items count: {}", ast.items.len());
            for (i, item) in ast.items.iter().enumerate() {
                eprintln!("  item[{}]: {:?}", i, item);
            }
        },
        Err(e) => eprintln!("AST 解析失败: {}", e),
    }

    // 测试 6: 实际渲染验证
    eprintln!("\n--- 渲染验证 ---");
    let mut adapter = nargo_template::DejaVuAdapter::new(DejaVuFrontend::new());

    // if 测试
    adapter.register_template("if_test", "<% if show %>visible<% end if %>").unwrap();
    let mut ctx = HashMap::new();
    ctx.insert("show".to_string(), NargoValue::Bool(true));
    let result = adapter.render("if_test", &NargoValue::Object(ctx)).unwrap();
    eprintln!("if (show=true) 结果: [{}]", result);
    if result.contains("visible") {
        eprintln!("  => 正确!");
    } else {
        eprintln!("  => 错误! 应该包含 'visible'");
    }

    // field 访问测试
    adapter.register_template("field_test", "<% loop field in items %><% field.key %>|<% end loop %>").unwrap();
    let mut ctx2 = HashMap::new();
    let mut item1 = HashMap::new();
    item1.insert("key".to_string(), NargoValue::String("hello".to_string()));
    ctx2.insert("items".to_string(), NargoValue::Array(vec![NargoValue::Object(item1)]));
    let result2 = adapter.render("field_test", &NargoValue::Object(ctx2)).unwrap();
    eprintln!("field.key 结果: [{}]", result2);
    if result2.contains("hello") {
        eprintln!("  => 正确!");
    } else {
        eprintln!("  => 错误! 应该包含 'hello'");
    }
}
