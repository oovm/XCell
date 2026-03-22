
use serde::Serialize;
use xcell_core::XResult;

#[derive(Clone, Debug, Default, Serialize)]
pub struct BinaryCodegen {
    /// Whether to generate binary data
    pub enable: bool,
    /// Output directory
    pub output: String,
}

impl BinaryCodegen {
    /// Binary output directory
    pub fn binary_path(&self, root: &std::path::Path, file_name: &str) -> XResult<std::path::PathBuf> {
        let dir = root.join(&self.output);
        let path = dir.join(file_name).with_extension("bin");
        Ok(path)
    }

    /// Binary relative path
    pub fn binary_relative(&self, file_name: &str) -> String {
        format!("{}/{}.bin", self.output, file_name)
    }

    /// Ensure output directories exist
    pub fn ensure_path(&self, root: &std::path::Path) -> XResult<()> {
        if self.enable {
            if let Some(s) = self.binary_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
        }
        Ok(())
    }
}

impl super::Codegen for BinaryCodegen {
    fn generate(&self, _context: &super::CodegenContext) -> XResult<()> {
        // TODO: Implement binary data generation
        Ok(())
    }

    fn name(&self) -> &'static str {
        "binary"
    }
}