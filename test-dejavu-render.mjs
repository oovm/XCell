import { spawn } from "node:child_process";
import { writeFileSync, mkdirSync, rmSync, existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dirname, "..");
const TEST_DIR = join(ROOT, "test-render-output");

function runWithTimeout(command, args, cwd, timeout = 60_000) {
    return new Promise((resolve, reject) => {
        const proc = spawn(command, args, { cwd, shell: true, stdio: "pipe" });
        let stdout = "";
        let stderr = "";
        proc.stdout.on("data", (d) => { stdout += d; });
        proc.stderr.on("data", (d) => { stderr += d; });
        const timer = setTimeout(() => {
            proc.kill("SIGKILL");
            reject(new Error(`进程超时 (${timeout / 1000}s)`));
        }, timeout);
        proc.on("close", (code) => {
            clearTimeout(timer);
            if (code === 0) resolve({ stdout, stderr });
            else reject(new Error(`进程退出码: ${code}\n${stderr.slice(-3000)}`));
        });
        proc.on("error", (err) => { clearTimeout(timer); reject(err); });
    });
}

async function main() {
    console.log("=== DejaVu 模板渲染调试 ===\n");

    const testRustCode = `
use nargo_template::{DejaVuAdapter, DejaVuFrontend, UnifiedTemplateEngine};
use nargo_types::NargoValue;
use std::collections::HashMap;

fn main() {
    let mut adapter = DejaVuAdapter::new(DejaVuFrontend::new());

    // 测试简单插值
    let template1 = "Hello <% name %>!";
    adapter.register_template("test1", template1).unwrap();
    let mut ctx1 = HashMap::new();
    ctx1.insert("name".to_string(), NargoValue::String("World".to_string()));
    let result1 = adapter.render("test1", &NargoValue::Object(ctx1)).unwrap();
    println!("Test1 (simple interpolation): [{}]", result1);

    // 测试 loop 语法
    let template2 = "<% loop item in items %><% item %>,<% end loop %>";
    adapter.register_template("test2", template2).unwrap();
    let mut ctx2 = HashMap::new();
    ctx2.insert("items".to_string(), NargoValue::Array(vec![
        NargoValue::String("a".to_string()),
        NargoValue::String("b".to_string()),
        NargoValue::String("c".to_string()),
    ]));
    let result2 = adapter.render("test2", &NargoValue::Object(ctx2)).unwrap();
    println!("Test2 (loop): [{}]", result2);

    // 测试 if 语法
    let template3 = "<% if show %>visible<% end if %>";
    adapter.register_template("test3", template3).unwrap();
    let mut ctx3 = HashMap::new();
    ctx3.insert("show".to_string(), NargoValue::Bool(true));
    let result3 = adapter.render("test3", &NargoValue::Object(ctx3)).unwrap();
    println!("Test3 (if true): [{}]", result3);

    // 测试实际 BuildEnumerate 模板
    let template4 = r#"// 代码生成, 修改无效! (XCell <% compiler_version %>)

export enum <% class_name %> {
<% loop field in enumerate_ids %>
    <% field.key %> = <% field.value %>,
<% end loop %>
}"#;
    adapter.register_template("test4", template4).unwrap();
    let mut ctx4 = HashMap::new();
    ctx4.insert("compiler_version".to_string(), NargoValue::String("0.1.0".to_string()));
    ctx4.insert("class_name".to_string(), NargoValue::String("Quality".to_string()));
    let mut ids = Vec::new();
    for (k, v) in [("Common", "0"), ("Uncommon", "1"), ("Rare", "2")] {
        let mut item = HashMap::new();
        item.insert("key".to_string(), NargoValue::String(k.to_string()));
        item.insert("value".to_string(), NargoValue::String(v.to_string()));
        ids.push(NargoValue::Object(item));
    }
    ctx4.insert("enumerate_ids".to_string(), NargoValue::Array(ids));
    let result4 = adapter.render("test4", &NargoValue::Object(ctx4)).unwrap();
    println!("Test4 (BuildEnumerate):\\n{}", result4);
}
`;

    // 写入测试 Rust 文件
    const testFile = join(ROOT, "test_dejavu_render.rs");
    writeFileSync(testFile, testRustCode);
    console.log("已写入测试文件:", testFile);

    // 编译并运行
    try {
        console.log("\n编译测试程序...");
        const buildResult = await runWithTimeout("cargo", [
            "build",
            "-p", "xcell-generator",
            "--features", "dejavu",
            "--message-format=short"
        ], ROOT);
        console.log("编译成功");

        // 使用 cargo run 执行测试
        console.log("\n运行测试...");
        const runResult = await runWithTimeout("cargo", [
            "run",
            "-p", "xcell-generator",
            "--features", "dejavu",
            "--",
            "--workspace", "examples/roguelike"
        ], ROOT, 120_000);
        console.log("stdout:", runResult.stdout.slice(-2000));
        if (runResult.stderr.trim()) {
            const lines = runResult.stderr.trim().split("\n").filter(l =>
                l.includes("error") || l.includes("warning") || l.includes("渲染") || l.includes("模板") || l.includes("dejavu")
            );
            if (lines.length > 0) {
                console.log("stderr 摘要:");
                for (const line of lines.slice(0, 20)) {
                    console.log("  " + line.trim());
                }
            }
        }
    } catch (err) {
        console.error("执行失败:", err.message);
    }

    // 清理
    try { rmSync(testFile, { force: true }); } catch {}
}

main();
