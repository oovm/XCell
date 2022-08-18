use xcell_types::VectorDescription;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_vector() {
    let vector = VectorDescription::default();
    assert_eq!(vector.matches_rest("i32[]"), Some("i32"))
}
