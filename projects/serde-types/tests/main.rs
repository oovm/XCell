use serde::{Deserialize, __private::de::Content};
use serde_json::from_str;

use serde_types::{OneOrMany, ParsableValue};

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

#[derive(Debug, Default, Deserialize)]
pub struct Test {
    id: String,
    user: u64,
    book: u64,
}
#[derive(Debug, Deserialize)]
pub enum TestEnum {
    AAA = 2,
    BBB = 3,
}

#[test]
fn test() {
    // let q = ParsableValue::new(Content::Seq(vec![Content::Str("255"), Content::Str("255")]));
    let mut q = ParsableValue::new(Content::I8(2));
    // q.insert("id", Content::Str("128"));
    // q.insert("user", Content::String("256".to_string()));
    // q.insert("book", Content::String("1024".to_string()));
    // println!("{:#?}", q);
    println!("{:#?}", TestEnum::deserialize(q).unwrap());
}
