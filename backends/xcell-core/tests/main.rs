use xcell_types::{
    ListDescription, ReferenceDescription, StringDescription, TypeMetaInfo, VectorDescription,
    XCellTyped, XCellValue,
};
use xcell_types::for_3rd::Data;

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

    let ref_type2 = XCellTyped::parse("ref<Monster>", &info);
    assert!(ref_type2.is_reference());
    assert_eq!(ref_type2.as_reference().unwrap().target_table, "Monster");

    let ref_type3 = XCellTyped::parse("Ref<Weapon>", &info);
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
