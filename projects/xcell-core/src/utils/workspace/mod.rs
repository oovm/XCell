use super::*;
use xcell_errors::for_3rd::DirEntry;
pub fn valid_file(dir: &DirEntry) -> bool {
    if !dir.path().is_file() {
        return false;
    }
    let file_name = dir.file_name();
    let file_name = file_name.to_string_lossy();
    if file_name.starts_with("~$") {
        return false;
    }
    true
}

/// 取得相对路径
///
/// # Arguments
///
/// * `this`:
/// * `root`:
///
/// # Examples
///
/// ```
/// use xcell_core;
/// ```
pub fn make_relative<A, B>(this: A, root: B) -> XResult<PathBuf>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = this.as_ref().canonicalize()?;
    let base = root.as_ref().canonicalize()?;
    match diff_paths(&path, &base) {
        #[cfg(target_os = "windows")]
        Some(o) => Ok(PathBuf::from(o.as_os_str().to_string_lossy().split("\\").join("/"))),
        #[cfg(not(target_os = "windows"))]
        Some(o) => Ok(o),
        None => Err(XError::table_error(format!("无法取得相对路径 {path:?} {base:?}"))),
    }
}
