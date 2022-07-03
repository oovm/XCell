use super::*;

pub fn walk_blob_set(root: &Path, glob: &GlobSet) -> XResult<Vec<PathBuf>> {
    let root = root.canonicalize()?;
    info!("工作目录: {}", root.display());
    let mut out = vec![];
    for entry in WalkDir::new(&root).follow_links(true) {
        let file = match entry {
            Ok(o) if valid_file(&o) => o.into_path(),
            _ => continue,
        };
        let normed = make_relative(&file, &root)?;
        if glob.is_match(&normed) {
            info!("首次加载: {}", normed.display());
            out.push(file)
        }
    }
    Ok(out)
}

pub fn valid_file(dir: &DirEntry) -> bool {
    if !dir.path().is_file() {
        return false;
    }
    let file_name = dir.file_name().to_string_lossy();
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
