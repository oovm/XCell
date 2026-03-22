use xcell_parser::parser::{TypeParser, TypeExpr, FieldExpr, MetaExpr, FieldConstraint, TableKind}; use xcell_parser::lexer::Lexer; use xcell_parser::error::ParseResult;

fn parse_type(input: &str) -> ParseResult<TypeExpr> {
    let lexer = Lexer::new(input);
    let tokens: Vec<xcell_parser::lexer::Token> = lexer.collect::<Result<_, _>>()?;
    let mut parser = TypeParser::new(tokens);
    parser.parse_type_expr()
}

fn parse_field(input: &str) -> ParseResult<FieldExpr> {
    let lexer = Lexer::new(input);
    let tokens: Vec<xcell_parser::lexer::Token> = lexer.collect::<Result<_, _>>()?;
    let mut parser = TypeParser::new(tokens);
    parser.parse_field_expr()
}

fn parse_meta(input: &str) -> ParseResult<MetaExpr> {
    let lexer = Lexer::new(input);
    let tokens: Vec<xcell_parser::lexer::Token> = lexer.collect::<Result<_, _>>()?;
    let mut parser = TypeParser::new(tokens);
    parser.parse_meta_expr()
}

#[test]
fn test_primitive_types() {
    assert!(matches!(parse_type("i32").unwrap(), TypeExpr::Primitive(xcell_parser::parser::PrimitiveType::I32)));
    assert!(matches!(parse_type("bool").unwrap(), TypeExpr::Primitive(xcell_parser::parser::PrimitiveType::Bool)));
    assert!(matches!(parse_type("string").unwrap(), TypeExpr::Primitive(xcell_parser::parser::PrimitiveType::String)));
}

#[test]
fn test_reference_type() {
    let ty = parse_type("&Item").unwrap();
    assert!(matches!(ty, TypeExpr::Reference { target } if target == "Item"));
}

#[test]
fn test_list_type() {
    let ty = parse_type("[i32]").unwrap();
    assert!(matches!(ty, TypeExpr::List { .. }));
}

#[test]
fn test_fixed_array() {
    let ty = parse_type("[i32; 5]").unwrap();
    match ty {
        TypeExpr::FixedArray { element, length } => {
            assert!(matches!(*element, TypeExpr::Primitive(xcell_parser::parser::PrimitiveType::I32)));
            assert_eq!(length, 5);
        }
        _ => panic!("Expected FixedArray type"),
    }
}

#[test]
fn test_reference_list() {
    let ty = parse_type("[&Item]").unwrap();
    match ty {
        TypeExpr::List { element } => {
            assert!(matches!(*element, TypeExpr::Reference { target } if target == "Item"));
        }
        _ => panic!("Expected List of Reference type"),
    }
}

#[test]
fn test_vec_type() {
    let ty = parse_type("Vec<i32>").unwrap();
    assert!(matches!(ty, TypeExpr::Vec { .. }));
}

#[test]
fn test_field_without_constraint() {
    let field = parse_field("item_id").unwrap();
    assert_eq!(field.name, "item_id");
    assert!(field.constraint.is_none());
}

#[test]
fn test_field_with_unique() {
    let field = parse_field("@email").unwrap();
    assert_eq!(field.name, "email");
    assert!(matches!(field.constraint, Some(FieldConstraint::Unique)));
}

#[test]
fn test_field_with_primary() {
    let field = parse_field("@@item_id").unwrap();
    assert_eq!(field.name, "item_id");
    assert!(matches!(field.constraint, Some(FieldConstraint::Primary)));
}

#[test]
fn test_meta_dict() {
    let meta = parse_meta("@dict").unwrap();
    assert!(matches!(meta.kind, TableKind::Dict));
    assert!(meta.unique_fields.is_empty());
}

#[test]
fn test_meta_class() {
    let meta = parse_meta("@class").unwrap();
    assert!(matches!(meta.kind, TableKind::Class));
}

#[test]
fn test_meta_with_unique_fields() {
    let meta = parse_meta("@dict @unique(class, level)").unwrap();
    assert!(matches!(meta.kind, TableKind::Dict));
    assert_eq!(meta.unique_fields, vec!["class", "level"]);
}

#[test]
fn test_display() {
    assert_eq!(parse_type("i32").unwrap().to_string(), "i32");
    assert_eq!(parse_type("&Item").unwrap().to_string(), "&Item");
    assert_eq!(parse_type("[i32]").unwrap().to_string(), "[i32]");
}
