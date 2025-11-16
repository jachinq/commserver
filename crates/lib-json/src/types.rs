use std::fmt::Display;

use crate::{list::JsonList, object::JsonObject, var::StringExt};

#[derive(Clone, Debug, PartialEq)]
pub struct Wrap<T>(pub T);

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISIZE(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USIZE(usize),
    F32(f32),
    F64(f64),
    String(String),
    Boolean(bool),
    JsonObject(JsonObject),
    JsonList(JsonList),
    Null,
}

impl Type {
    /// 解析字符串为 type 类型
    pub fn parse_type(value: &str) -> Option<Type> {
        let v_string = value.trim().to_lowercase();
        let v = v_string.as_str();
        match v {
            "true" | "false" => match v.parse() {
                Ok(value) => Some(Type::Boolean(value)),
                Err(e) => {
                    println!("parse_type boolean error:{} value:{}", e, v);
                    Some(Type::String(value.to_string()))
                }
            },
            "null" | "nan" => Some(Type::Null),
            _ if v_string.index_of(".").is_some() => match v.parse() {
                Ok(value) => Some(Type::F64(value)),
                Err(e) => {
                    println!("parse_type f64 error:{} value:{}", e, v);
                    Some(Type::String(value.to_string()))
                }
            },
            _ if v.starts_with("{") && v.ends_with("}") => {
                value.trim().parse().ok().map(Type::JsonObject)
            }
            _ if v.starts_with("[") && v.ends_with("]") => {
                value.trim().parse().ok().map(Type::JsonList)
            }
            _ => match v.parse() {
                Ok(value) => Some(Type::ISIZE(value)),
                Err(_) => Some(Type::String(value.to_string())),
            },
        }
    }

    pub fn to_json(&self) -> String {
        self.to_string()
    }

    pub(crate) fn to_json_line(&self, indent: usize) -> String {
        match self {
            Type::JsonObject(o) => o.to_json_line_with_indent(indent),
            Type::JsonList(l) => l.to_json_line_with_indent(indent),
            _ => self.to_json(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Type::String(s) => format!("\"{}\"", s),
            Type::I8(i) => i.to_string(),
            Type::I16(i) => i.to_string(),
            Type::I32(i) => i.to_string(),
            Type::I64(i) => i.to_string(),
            Type::I128(i) => i.to_string(),
            Type::ISIZE(i) => i.to_string(),
            Type::U8(u) => u.to_string(),
            Type::U16(u) => u.to_string(),
            Type::U32(u) => u.to_string(),
            Type::U64(u) => u.to_string(),
            Type::U128(u) => u.to_string(),
            Type::USIZE(u) => u.to_string(),
            Type::F32(value) => {
                if value.fract() == 0.0 {
                    format!("{:.1}", value) // 整数情况，打印1位小数
                } else {
                    format!("{}", value) // 非整数情况，打印完整
                }
            }
            Type::F64(value) => {
                if value.fract() == 0.0 {
                    format!("{:.1}", value) // 整数情况，打印1位小数
                } else {
                    format!("{}", value) // 非整数情况，打印完整
                }
            }
            Type::Boolean(b) => b.to_string(),
            Type::JsonObject(o) => o.to_json(),
            Type::JsonList(l) => l.to_json(),
            Type::Null => "null".to_string(),
        };
        f.write_str(&s)
    }
}
