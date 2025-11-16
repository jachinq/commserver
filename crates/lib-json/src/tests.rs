#![cfg(test)]

use std::isize;

use crate::{list::JsonList, object::JsonObject, types::Type};

fn parse_object(json: &str) -> Result<JsonObject, crate::error::Error> {
    json.parse()
}

fn parse_list(json: &str) -> Result<JsonList, crate::error::Error> {
    let list = JsonList::parse(json.to_string());
    if list.is_none() {
        return Err(crate::error::Error::InvalidJson);
    }
    Ok(list.unwrap())
}

#[test]
fn test_parse_object() {
    // gen random json strings from https://www.lddgo.net/string/random-json
    assert!(parse_object("{}").is_ok());
    assert!(parse_object("{\"a\": 1}").is_ok());
    assert!(parse_object(r#""#).is_err());
    assert!(parse_object(r#"{}"#).is_ok());

    let json = r#"{"name":"fugiat ea tempor laborum","age":-19103712.601825505,"address":{"consequate":-37661686.80219111,"sedcf5":true,"inc89":"sed exercitation nisi in"}}"#;
    assert!(parse_object(json).is_ok());

    let json = r#"
        {
            "string": "John Doe\u554a",
            "number": 3.14,
            "boolean": true,
            "array": [1, 2, 3],
            "object": {
                "a": 1,
                "object": {"b": 3}
            }
        }"#;
    println!("{:?}", parse_object(json)); // cargo test -- --nocapture
    assert!(parse_object(json).is_ok());

    let json = r#"
        {
            "list": [
                {
                    "name": "John Doe",
                    "age": 18
                },
                {
                    "name": "Jane Doe",
                    "age": 20
                }
            ]
        }
        "#;
    assert!(parse_object(json).is_ok());
}

#[test]
fn test_parse_list() {
    assert!(parse_list("[]").is_ok());
    assert!(parse_list("[1, 2, 3]").is_ok());
    assert!(parse_list(r#"[1, 2, 3]"#).is_ok());

    let json = r#"[{"name": "John Doe","age": 18},{"name": "Jane Doe","age": 20}]"#;
    assert!(parse_list(json).is_ok());

    let json = "[{\"name\":\"John Doe\",\"age\":18},{\"name\":\"Jane Doe\",\"age\":20}]";
    assert!(parse_list(json).is_ok());

    let json = r#"[
            {
                "name": "John Doe",
                "age": 18
            },
            {
                "name": "Jane Doe",
                "age": 20
            }
        ]"#;
    // println!("list = {:#?}", parse_list(json).unwrap());
    assert!(parse_list(json).is_ok());
}

#[test]
fn test_parse_type() {
    let max = isize::MAX;
    let max_str = format!("{}9", max);
    assert_eq!(Type::ISIZE(12), Type::parse_type("12").unwrap());
    assert_eq!(Type::F64(64.0), Type::parse_type("64.0").unwrap());
    assert_eq!(Type::F64(-12.4421), Type::parse_type("-12.4421").unwrap());
    assert_eq!(Type::String("John Doe".to_string()), Type::parse_type("John Doe").unwrap());
    assert_eq!(Type::Boolean(false), Type::parse_type("false").unwrap());
    assert_eq!(Type::Boolean(true), Type::parse_type("true").unwrap());
    assert_eq!(Type::Null, Type::parse_type("null").unwrap());
    assert_eq!(Type::Null, Type::parse_type("nan").unwrap());
    assert_eq!(Type::ISIZE(max), Type::parse_type(&max.to_string()).unwrap());
    assert_eq!(Type::String(max_str.clone()), Type::parse_type(&max_str).unwrap());

    let object_json = r#"{"name":"John Doe","age":18}"#;
    let object = parse_object(object_json).unwrap();
    assert_eq!(Type::JsonObject(object), Type::parse_type(object_json).unwrap());
    
    let list_json = r#"[{"name":"John Doe","age":18},{"name":"John Doe","age":20}]"#;
    let list = parse_list(&list_json).unwrap();
    assert_eq!(Type::JsonList(list), Type::parse_type(list_json).unwrap());
}


#[test]
fn test_object_to_json_line() {
    let json = r#"{
    "name": "John Doe",
    "age": 18,
    "list": [
        1,
        2,
        4
    ]
}"#;

    let object = parse_object(json).unwrap();
    let json_line = object.to_json_line();
    assert_eq!(json.to_string(), json_line);
}
