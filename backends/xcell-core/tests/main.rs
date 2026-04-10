use xcell_core::{
    ListDescription, MapDescription, OptionalDescription, ReferenceDescription, StringDescription,
    TypeMetaInfo, VectorDescription, XCellTyped, XCellValue, XError, XErrorKind,
};
use xcell_core::for_3rd::Data;

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
    assert!(string.matches_type("string"));
}

#[test]
fn test_reference_type_parsing() {
    let info = TypeMetaInfo::default();

    let ref_type = XCellTyped::parse("&Item", &info);
    assert!(ref_type.is_reference());
    assert_eq!(ref_type.as_reference().unwrap().target_table, "Item");

    let ref_type2 = XCellTyped::parse("&Monster", &info);
    assert!(ref_type2.is_reference());
    assert_eq!(ref_type2.as_reference().unwrap().target_table, "Monster");

    let ref_type3 = XCellTyped::parse("&Weapon", &info);
    assert!(ref_type3.is_reference());
    assert_eq!(ref_type3.as_reference().unwrap().target_table, "Weapon");
}

#[test]
fn test_list_type_parsing() {
    let info = TypeMetaInfo::default();

    let list_type = XCellTyped::parse("[i32]", &info);
    assert!(list_type.is_list());
    assert!(list_type.as_list().unwrap().fixed_length.is_none());

    let ref_list = XCellTyped::parse("[&Item]", &info);
    assert!(ref_list.is_list());
    let list_desc = ref_list.as_list().unwrap();
    assert!(list_desc.element_type.is_reference());
    assert_eq!(list_desc.element_type.as_reference().unwrap().target_table, "Item");

    let fixed_list = XCellTyped::parse("[i32; 5]", &info);
    assert!(fixed_list.is_list());
    let list_desc = fixed_list.as_list().unwrap();
    assert_eq!(list_desc.fixed_length, Some(5));
}

#[test]
fn test_nested_list_type_parsing() {
    let info = TypeMetaInfo::default();

    let str_list = XCellTyped::parse("[string]", &info);
    assert!(str_list.is_list());

    let enum_list = XCellTyped::parse("[Rarity]", &info);
    assert!(enum_list.is_list());
}

#[test]
fn test_reference_cell_parsing() {
    let ref_desc = ReferenceDescription::new("Item");
    let cell = Data::Int(5);
    let value = ref_desc.parse_cell(&cell).unwrap();
    assert!(matches!(value, XCellValue::Reference(5)));

    let cell_str = Data::String("10".to_string());
    let value_str = ref_desc.parse_cell(&cell_str).unwrap();
    assert!(matches!(value_str, XCellValue::Reference(10)));

    let cell_empty = Data::Empty;
    let value_empty = ref_desc.parse_cell(&cell_empty);
    assert!(value_empty.is_err());
}

#[test]
fn test_reference_cell_parsing_with_default() {
    let mut ref_desc = ReferenceDescription::new("Item");
    ref_desc.default = Some(0);

    let cell_empty = Data::Empty;
    let value = ref_desc.parse_cell(&cell_empty).unwrap();
    assert!(matches!(value, XCellValue::Reference(0)));
}

#[test]
fn test_list_cell_parsing() {
    let info = TypeMetaInfo::default();
    let list_desc = ListDescription {
        element_type: XCellTyped::parse("i32", &info),
        ..Default::default()
    };

    let cell = Data::String("1,2,3".to_string());
    let value = list_desc.parse_cell(&cell).unwrap();
    match value {
        XCellValue::Vector(items) => {
            assert_eq!(items.len(), 3);
            assert!(matches!(items[0], XCellValue::Integer32(1)));
            assert!(matches!(items[1], XCellValue::Integer32(2)));
            assert!(matches!(items[2], XCellValue::Integer32(3)));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_list_cell_parsing_empty() {
    let info = TypeMetaInfo::default();
    let list_desc = ListDescription {
        element_type: XCellTyped::parse("i32", &info),
        ..Default::default()
    };

    let cell = Data::String("".to_string());
    let value = list_desc.parse_cell(&cell).unwrap();
    match value {
        XCellValue::Vector(items) => {
            assert_eq!(items.len(), 0);
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_list_cell_parsing_fixed_length() {
    let info = TypeMetaInfo::default();
    let list_desc = ListDescription {
        element_type: XCellTyped::parse("i32", &info),
        fixed_length: Some(3),
        ..Default::default()
    };

    let cell = Data::String("1,2,3".to_string());
    let value = list_desc.parse_cell(&cell).unwrap();
    match value {
        XCellValue::Vector(items) => {
            assert_eq!(items.len(), 3);
        }
        _ => panic!("Expected Vector"),
    }

    let cell_invalid = Data::String("1,2".to_string());
    let result = list_desc.parse_cell(&cell_invalid);
    assert!(result.is_err());
}

#[test]
fn test_list_cell_parsing_reference_elements() {
    let info = TypeMetaInfo::default();
    let list_desc = ListDescription {
        element_type: XCellTyped::parse("&Item", &info),
        ..Default::default()
    };

    let cell = Data::String("1,2,3".to_string());
    let value = list_desc.parse_cell(&cell).unwrap();
    match value {
        XCellValue::Vector(items) => {
            assert_eq!(items.len(), 3);
            assert!(matches!(items[0], XCellValue::Reference(1)));
            assert!(matches!(items[1], XCellValue::Reference(2)));
            assert!(matches!(items[2], XCellValue::Reference(3)));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_vec_type_parsing() {
    let info = TypeMetaInfo::default();

    let vec_type = XCellTyped::parse("Vec<i32>", &info);
    let _ = vec_type;
}

#[test]
fn test_map_type_parsing() {
    let info = TypeMetaInfo::default();

    let map_type = XCellTyped::parse("Map<string, i32>", &info);
    assert!(map_type.is_map());
    let map_desc = map_type.as_map().unwrap();
    assert!(map_desc.key_type.is_list() || map_desc.key_type.as_reference().is_none());
}

#[test]
fn test_optional_type_parsing() {
    let info = TypeMetaInfo::default();

    let opt_type = XCellTyped::parse("i32?", &info);
    assert!(opt_type.is_optional());
    let opt_desc = opt_type.as_optional().unwrap();
    assert!(matches!(opt_desc.element_type, XCellTyped::Integer(_)));
}

#[test]
fn test_map_cell_parsing() {
    let info = TypeMetaInfo::default();
    let map_desc = MapDescription {
        key_type: XCellTyped::parse("string", &info),
        value_type: XCellTyped::parse("i32", &info),
        ..Default::default()
    };

    let cell = Data::String("a:1,b:2".to_string());
    let value = map_desc.parse_cell(&cell).unwrap();
    match value {
        XCellValue::Map(map) => {
            assert_eq!(map.len(), 2);
            assert!(map.contains_key("a"));
            assert!(map.contains_key("b"));
        }
        _ => panic!("Expected Map"),
    }
}

#[test]
fn test_optional_cell_parsing_empty() {
    let info = TypeMetaInfo::default();
    let opt_desc = OptionalDescription {
        element_type: XCellTyped::parse("i32", &info),
        ..Default::default()
    };

    let cell = Data::Empty;
    let value = opt_desc.parse_cell(&cell).unwrap();
    match value {
        XCellValue::Optional(inner) => {
            assert!(inner.is_none());
        }
        _ => panic!("Expected Optional"),
    }
}

#[test]
fn test_optional_cell_parsing_value() {
    let info = TypeMetaInfo::default();
    let opt_desc = OptionalDescription {
        element_type: XCellTyped::parse("i32", &info),
        ..Default::default()
    };

    let cell = Data::String("42".to_string());
    let value = opt_desc.parse_cell(&cell).unwrap();
    match value {
        XCellValue::Optional(Some(inner)) => {
            assert!(matches!(*inner, XCellValue::Integer32(42)));
        }
        _ => panic!("Expected Optional(Some)"),
    }
}

#[test]
fn test_type_meta_info_complete() {
    let info = TypeMetaInfo::default();
    let _ = &info.boolean;
    let _ = &info.string;
    let _ = &info.vector;
    let _ = &info.language;
    let _ = &info.enumerate;
    let _ = &info.decimal;
    let _ = &info.time;
    let _ = &info.color;
    let _ = &info.reference;
}

#[test]
fn test_parse_error_conversion() {
    let parse_err = xcell_parser::ParseError::new(
        xcell_parser::ParseErrorKind::InvalidTypeName("bad".to_string()),
        0,
    );
    let xerror: XError = parse_err.into();
    match &*xerror.kind {
        XErrorKind::ParseError { message, .. } => {
            assert!(!message.is_empty());
        }
        _ => panic!("Expected ParseError variant"),
    }
}
