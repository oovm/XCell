use xcell_errors::{
    for_3rd::{Glob, GlobSetBuilder},
    XResult,
};

#[test]
fn test_blob() -> XResult<()> {
    let mut builder = GlobSetBuilder::new();
    // A GlobBuilder can be used to configure each glob's match semantics
    // independently.
    builder.add(Glob::new("*.rs")?);
    builder.add(Glob::new("src/lib.rs")?);
    builder.add(Glob::new("src/**/foo.rs")?);
    let set = builder.build()?;

    assert_eq!(set.matches("src/bar/baz/foo.rs"), vec![0, 2]);
    Ok(())
}
