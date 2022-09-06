use xcell_types::{StringDescription, VectorDescription};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_vector() {
    let mut vector = VectorDescription::default();
    assert_eq!(vector.matches_rest("i32[]"), Some("i32"));
    vector.add_suffix("Array");
    assert_eq!(vector.matches_rest("i32Array"), Some("i32"));
    assert_eq!(vector.matches_rest("i32array"), Some("i32"));
}

#[test]
fn test_string() {
    let string = StringDescription::default();
    assert!(string.matches_type("str"));
    assert!(!string.matches_type("key"));
    assert!(string.matches_type("class"));
}
