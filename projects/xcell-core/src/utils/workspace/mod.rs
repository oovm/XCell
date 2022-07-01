use super::*;


pub fn build_glob(patterns: &str) -> XResult<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for line in patterns.lines() {
        builder.add(Glob::new(line)?);
    }
    let set = builder.build()?;
    Ok(set)
}
