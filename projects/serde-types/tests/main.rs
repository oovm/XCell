use serde_types::OneOrMany;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn one_or_many() {
    let out = serde_json::from_str::<OneOrMany<u8>>(
        r#"
[1]
    "#,
    )
    .unwrap();
    println!("{:#?}", out)
}
