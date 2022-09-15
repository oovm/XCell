use super::*;

impl XCellArgs {
    pub fn resolve_workspace(&self) -> XResult<PathBuf> {
        if self.workspace.is_empty() {
            return Ok(current_dir()?);
        }
        let path = PathBuf::from(&self.workspace);
        if path.is_absolute() {
            return Ok(path);
        }
        let absolute = current_dir()?.join(path).canonicalize()?;
        Ok(absolute)
    }
}
