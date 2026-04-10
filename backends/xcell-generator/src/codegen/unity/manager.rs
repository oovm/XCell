use super::*;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use xcell_analyzer::WorkspaceManager;
use xcell_config::UnityCodegen;
use xcell_core::XResult;

/// Unity 管理器代码生成模板数据
pub struct UnityManagerTemplate {
    /// 编译器版本
    pub compiler_version: &'static str,
    /// 数据版本
    pub data_version: String,
    /// 编辑时间
    pub edit_time: String,
    /// Unity 代码生成配置
    pub config: UnityCodegen,
    /// 表列表
    pub tables: Vec<TableField>,
}

/// 管理器中的表字段信息
#[derive(Clone, Debug)]
pub struct TableField {
    /// 表类型名
    pub typing: String,
    /// 公开属性名
    pub public_name: String,
    /// 私有字段名
    pub private_name: String,
}

impl Display for TableField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityGenerator {
    /// 写入 Unity 管理器代码
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 操作结果
    pub(super) fn write_manager(&self, ws: &WorkspaceManager) -> XResult<()> {
        let root = &ws.config.root;
        let output_dir = self.config.loader_path(root);

        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let path = output_dir.join(format!("{}.cs", self.config.manager));
        let mut file = std::fs::File::create(path)?;
        let template = make_manager(&self.config, ws);
        let out = render_manager(&template)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}

/// 创建 Unity 管理器模板数据
fn make_manager(config: &UnityCodegen, ws: &WorkspaceManager) -> UnityManagerTemplate {
    let mut tables = Vec::new();

    for table in ws.dicts() {
        let table_name = format!("{}{}", table.name, config.suffix_table);
        let private_name = format!("_{}", table.name.to_case(Case::Camel));
        let public_name = table.name.to_case(Case::Pascal);
        tables.push(TableField {
            typing: table_name,
            public_name,
            private_name,
        });
    }

    for table in ws.lists() {
        let table_name = format!("{}{}", table.name, config.suffix_table);
        let private_name = format!("_{}", table.name.to_case(Case::Camel));
        let public_name = table.name.to_case(Case::Pascal);
        tables.push(TableField {
            typing: table_name,
            public_name,
            private_name,
        });
    }

    UnityManagerTemplate {
        compiler_version: env!("CARGO_PKG_VERSION"),
        data_version: String::new(),
        edit_time: String::new(),
        config: config.clone(),
        tables,
    }
}

/// 渲染 Unity 管理器代码
fn render_manager(template: &UnityManagerTemplate) -> XResult<String> {
    let mut sb = String::new();
    sb.push_str(&format!("// 代码生成, 修改无效! (XCell {})\n", template.compiler_version));
    sb.push_str("#pragma warning disable\n\n");
    sb.push_str("using System;\nusing System.Collections.Generic;\nusing System.IO;\n\n");
    sb.push_str(&format!("namespace {}\n{{\n", template.config.namespace));
    sb.push_str(&format!("    public sealed class {}\n    {{\n", template.config.manager));

    for table in &template.tables {
        sb.push_str(&format!("        private {} {};\n", table.typing, table.private_name));
    }
    sb.push_str("\n");

    for table in &template.tables {
        sb.push_str(&format!("        public {} {}\n        {{\n            get\n            {{\n", table.typing, table.public_name));
        sb.push_str(&format!("                if ({} == null)\n                {{\n", table.private_name));
        sb.push_str(&format!("                    {} = new {}();\n", table.private_name, table.typing));
        sb.push_str(&format!("                    // TODO: 加载数据\n                }}\n"));
        sb.push_str(&format!("                return {};\n", table.private_name));
        sb.push_str("            }\n        }\n\n");
    }

    sb.push_str("        public void Reload()\n        {\n");
    for table in &template.tables {
        sb.push_str(&format!("            {} = null;\n", table.private_name));
    }
    sb.push_str("        }\n\n");

    sb.push_str("        public void Clear()\n        {\n");
    for table in &template.tables {
        sb.push_str(&format!("            {} = null;\n", table.private_name));
    }
    sb.push_str("        }\n");

    sb.push_str(&format!("\n        private static {} _instance;\n", template.config.manager));
    sb.push_str(&format!("        public static {} Instance\n        {{\n            get\n            {{\n", template.config.manager));
    sb.push_str(&format!("                if (_instance == null)\n                    _instance = new {}();\n", template.config.manager));
    sb.push_str("                return _instance;\n            }\n        }\n");

    sb.push_str("    }\n}\n");
    Ok(sb)
}
