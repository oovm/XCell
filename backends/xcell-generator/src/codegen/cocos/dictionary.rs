use super::*;
use xcell_analyzer::{XDictData, XListData};

impl CocosCodegen {
    /// Writes Cocos dictionary code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Dictionary data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_dict(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        let out = self.generate_dict_class_code(table.name.clone(), table);
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Writes Cocos list code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - List data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_list(&self, ws: &WorkspaceManager, table: &XListData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        let out = self.generate_list_class_code(table.name.clone(), table);
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Generates class code for dict tables
    ///
    /// # Arguments
    /// * `class_name` - Class name
    /// * `table` - Dictionary data table
    ///
    /// # Returns
    /// Generated class code
    fn generate_dict_class_code(&self, class_name: String, table: &XDictData) -> String {
        let mut code = String::new();
        
        // Header
        code.push_str(&format!("// 代码生成, 修改无效! (XCell {})\n\n\n\n", env!("CARGO_PKG_VERSION")));
        
        // Class definition
        code.push_str(&format!("export class {} {{
", class_name));
        
        // Fields
        for header in &table.headers {
            let default = header.typing.as_typescript_default();
            if !default.is_empty() {
                code.push_str(&format!("    {}: {} = {};\n", header.field_name, header.typing.as_typescript_type(), default));
            } else {
                code.push_str(&format!("    {}!: {};\n", header.field_name, header.typing.as_typescript_type()));
            }
        }
        
        // Closing brace
        code.push_str("}");
        
        code
    }

    /// Generates class code for list tables
    ///
    /// # Arguments
    /// * `class_name` - Class name
    /// * `table` - List data table
    ///
    /// # Returns
    /// Generated class code
    fn generate_list_class_code(&self, class_name: String, table: &XListData) -> String {
        let mut code = String::new();
        
        // Header
        code.push_str(&format!("// 代码生成, 修改无效! (XCell {})\n\n\n\n", env!("CARGO_PKG_VERSION")));
        
        // Class definition
        code.push_str(&format!("export class {} {{
", class_name));
        
        // Fields
        for header in &table.headers {
            let default = header.typing.as_typescript_default();
            if !default.is_empty() {
                code.push_str(&format!("    {}: {} = {};\n", header.field_name, header.typing.as_typescript_type(), default));
            } else {
                code.push_str(&format!("    {}!: {};\n", header.field_name, header.typing.as_typescript_type()));
            }
        }
        
        // Closing brace
        code.push_str("}");
        
        code
    }
}



