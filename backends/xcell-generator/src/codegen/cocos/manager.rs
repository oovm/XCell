use super::*;
use chrono;

impl CocosCodegen {
    /// Writes Cocos manager code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_manager(&self, ws: &WorkspaceManager) -> XResult<()> {
        let mut file = self.log_typescript(ws, &self.manager_name)?;
        let out = self.generate_manager_code(ws);
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Generates the manager code directly without using template engine
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Generated manager code
    fn generate_manager_code(&self, ws: &WorkspaceManager) -> String {
        let mut code = String::new();
        
        // Header
        code.push_str(&format!("// 代码生成, 修改无效! (XCell {})\n\n\n\n", env!("CARGO_PKG_VERSION")));
        
        // Documentation
        code.push_str("/**
 * 配置表管理器
 * 
 * 热更新资源直接 set 即可
 * 释放资源直接将表设为 null 即可
 */
");
        
        // Class definition
        code.push_str(&format!("export class {} {{
", self.manager_name));
        
        // Table version
        code.push_str("    /**
     * 配置表的版本号
     */
");
        code.push_str(&format!("    static readonly TableVersion = \"1.0.0\";

"));
        
        // Edit time
        code.push_str("    /**
     * 配置表的最后修改时间 (UTC)
     */
");
        code.push_str(&format!("    static readonly TableEditTime = new Date(\"{}\");

", chrono::Utc::now().to_rfc3339()));
        
        // Singleton instance
        code.push_str(&format!("    private static _instance: {} | null = null;\n", self.manager_name));
        code.push_str(&format!("    static get {}(): {} {{
", self.instance_name, self.manager_name));
        code.push_str(&format!("        if (!{}._instance) {{
", self.manager_name));
        code.push_str(&format!("            {}._instance = new {}();\n", self.manager_name, self.manager_name));
        code.push_str("        }\n");
        code.push_str(&format!("        return {}._instance;\n", self.manager_name));
        code.push_str("    }\n\n\n\n");
        
        // Table properties and getters/setters
        let tables = self.collect_tables(ws);
        for table in &tables {
            code.push_str(&format!("    private _{}: {} | null = null;\n", table.private_name, table.typing));
            code.push_str(&format!("    /** {} 表 */\n", table.typing));
            code.push_str(&format!("    get {}(): {} {{
", table.public_name, table.typing));
            code.push_str(&format!("        if (!this._{}) {{
", table.private_name));
            code.push_str(&format!("            this._{} = new {}();\n", table.private_name, table.typing));
            code.push_str("        }\n");
            code.push_str(&format!("        return this._{};\n", table.private_name));
            code.push_str("    }\n");
            code.push_str(&format!("    set {} (value: {}) {{
", table.public_name, table.typing));
            code.push_str(&format!("        this._{} = value;\n", table.private_name));
            code.push_str("    }\n");
        }
        
        // Reload method
        code.push_str("    /** 重新加载所有表 */\n");
        code.push_str("    static reload(): void {\n");
        for table in &tables {
            code.push_str(&format!("        {}[`{}`]._{} = null;\n", self.manager_name, self.instance_name, table.private_name));
        }
        code.push_str("    }\n\n");
        
        // Clear method
        code.push_str("    /** 清除所有表 */\n");
        code.push_str("    static clear(): void {\n");
        for table in &tables {
            code.push_str(&format!("        {}[`{}`]._{} = null;\n", self.manager_name, self.instance_name, table.private_name));
        }
        code.push_str("    }\n");
        
        // Closing brace
        code.push_str("}");
        
        code
    }

    /// Collects all tables from workspace
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Vector of table items
    fn collect_tables(&self, ws: &WorkspaceManager) -> Vec<TableItem> {
        let mut tables = Vec::new();
        
        // Add class tables
        for table in ws.classes() {
            let table_name = format!("{}{}", table.name, self.suffix_table);
            let private_name = format!("{}Table", table.name.to_lowercase());
            let public_name = format!("get{}Table", table.name);
            tables.push(TableItem {
                private_name,
                public_name,
                typing: table_name,
            });
        }
        
        // Add dict tables
        for table in ws.dicts() {
            let table_name = format!("{}{}", table.name, self.suffix_table);
            let private_name = format!("{}Table", table.name.to_lowercase());
            let public_name = format!("get{}Table", table.name);
            tables.push(TableItem {
                private_name,
                public_name,
                typing: table_name,
            });
        }
        
        // Add list tables
        for table in ws.lists() {
            let table_name = format!("{}{}", table.name, self.suffix_table);
            let private_name = format!("{}Table", table.name.to_lowercase());
            let public_name = format!("get{}Table", table.name);
            tables.push(TableItem {
                private_name,
                public_name,
                typing: table_name,
            });
        }
        
        tables
    }
}

/// Table item struct
#[derive(Debug, Clone)]
pub struct TableItem {
    /// Private table name
    pub private_name: String,
    /// Public table name
    pub public_name: String,
    /// Table typing
    pub typing: String,
}

