use xcell_types::{StringDescription, VectorDescription};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_vector() {
    let vector = VectorDescription::default();
    assert_eq!(vector.matches_rest("i32[]"), Some("i32"))
}

#[test]
fn test_string() {
    let string = StringDescription::default();
    println!("{:?}", string.matches_type("str"));
    // assert!(!string.matches_type("key"));
    // assert!(string.matches_type("string"));
}
