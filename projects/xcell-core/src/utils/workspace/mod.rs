use futures_lite::stream::StreamExt;

use super::*;

pub async fn walk_blob_set(root: &Path, glob: &GlobSet) -> XResult<Vec<PathBuf>> {
    let root = root.canonicalize()?;
    info!("工作目录: {}", root.display());
    let mut out = vec![];
    let mut entries = WalkDir::new(&root);
    loop {
        match entries.next().await {
            Some(Ok(o)) if valid_file(&o) => {
                let file = o.path();
                let normed = make_relative(&file, &root)?;
                if glob.is_match(&normed) {
                    info!("首次加载: {}", normed.display());
                    out.push(file)
                }
            }
            None => break,
            _ => continue,
        }
    }
    Ok(out)
}

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

pub fn build_glob_set(patterns: &str) -> XResult<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for line in patterns.lines() {
        builder.add(Glob::new(line)?);
    }
    let set = builder.build()?;
    Ok(set)
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
