use serde_json::from_str;
use serde_types::OneOrMany;
#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn one_or_many() {
    match from_str::<OneOrMany<u8>>("") {
        Ok(out) => panic!("{out}"),
        Err(e) => assert_eq!(e.to_string(), "EOF while parsing a value at line 1 column 0"),
    }
    match from_str::<OneOrMany<u8>>("true") {
        Ok(out) => panic!("{out}"),
        Err(e) => assert_eq!(e.to_string(), "`None` does not `u8` or sequence of `u8`"),
    }
    match from_str::<OneOrMany<u8>>("[]") {
        Ok(out) => assert_eq!(out, OneOrMany::empty()),
        Err(e) => panic!("{e}"),
    }
    match from_str::<OneOrMany<u8>>("0") {
        Ok(out) => assert_eq!(out, OneOrMany::one(0)),
        Err(e) => panic!("{e}"),
    }
    match from_str::<OneOrMany<u8>>("[0]") {
        Ok(out) => assert_eq!(out, OneOrMany::new([0])),
        Err(e) => panic!("{e}"),
    }
}
