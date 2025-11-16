use crate::{
    list::JsonList,
    pair::Pair,
    types::Type,
    var::{self, StringExt},
};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, PartialEq)]
pub struct JsonObject {
    list: Vec<Pair>,
}

impl JsonObject {
    pub fn new() -> Self {
        Self {
            list: Vec::with_capacity(0),
        }
    }

    pub fn set_data(&mut self, key: &str, value: Type) -> &Self {
        self.list.push(Pair::new(key, value));
        self
    }
    pub fn get_data(&self, key: &str) -> Option<&Type> {
        for pair in &self.list {
            if pair.first == key {
                return Some(&pair.second);
            }
        }
        None
    }

    pub fn get_integer<T: TryFrom<isize>>(&self, key: &str) -> Option<T> {
        let types = self.get_data(key);
        if let Some(value) = types {
            return match *value {
                Type::I8(v) => T::try_from(v as isize).ok(),
                Type::I16(v) => T::try_from(v as isize).ok(),
                Type::I32(v) => T::try_from(v as isize).ok(),
                Type::I64(v) => T::try_from(v as isize).ok(),
                Type::I128(v) => T::try_from(v as isize).ok(),
                Type::ISIZE(v) => T::try_from(v as isize).ok(),
                _ => None,
            };
        }
        None
    }

    pub fn get_unsign<T: TryFrom<usize>>(&self, key: &str) -> Option<T> {
        if let Some(value) = self.get_data(key) {
            return match *value {
                Type::U8(u) => T::try_from(u as usize).ok(),
                Type::U16(u) => T::try_from(u as usize).ok(),
                Type::U32(u) => T::try_from(u as usize).ok(),
                Type::U64(u) => T::try_from(u as usize).ok(),
                Type::U128(u) => T::try_from(u as usize).ok(),
                Type::USIZE(u) => T::try_from(u as usize).ok(),
                _ => None,
            };
        }
        None
    }

    pub fn set_i8(&mut self, key: &str, value: i8) -> &Self {
        self.set_data(key, Type::I8(value))
    }
    pub fn get_i8(&self, key: &str) -> Option<i8> {
        self.get_integer(key)
    }

    pub fn set_i16(&mut self, key: &str, value: i16) -> &Self {
        self.set_data(key, Type::I16(value))
    }
    pub fn get_i16(&self, key: &str) -> Option<i16> {
        self.get_integer(key)
    }

    pub fn set_i32(&mut self, key: &str, value: i32) -> &Self {
        self.set_data(key, Type::I32(value))
    }
    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.get_integer(key)
    }

    pub fn set_i64(&mut self, key: &str, value: i64) -> &Self {
        self.set_data(key, Type::I64(value))
    }
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get_integer(key)
    }

    pub fn set_i128(&mut self, key: &str, value: i128) -> &Self {
        self.set_data(key, Type::I128(value))
    }
    pub fn get_i128(&self, key: &str) -> Option<i128> {
        self.get_integer(key)
    }

    pub fn set_isize(&mut self, key: &str, value: isize) -> &Self {
        self.set_data(key, Type::ISIZE(value))
    }
    pub fn get_isize(&self, key: &str) -> Option<isize> {
        self.get_integer(key)
    }

    pub fn set_u8(&mut self, key: &str, value: u8) -> &Self {
        self.set_data(key, Type::U8(value))
    }
    pub fn get_u8(&self, key: &str) -> Option<u8> {
        self.get_unsign(key)
    }

    pub fn set_u16(&mut self, key: &str, value: u16) -> &Self {
        self.set_data(key, Type::U16(value))
    }
    pub fn get_u16(&self, key: &str) -> Option<u16> {
        self.get_unsign(key)
    }

    pub fn set_u32(&mut self, key: &str, value: u32) -> &Self {
        self.set_data(key, Type::U32(value))
    }
    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.get_unsign(key)
    }

    pub fn set_u64(&mut self, key: &str, value: u64) -> &Self {
        self.set_data(key, Type::U64(value))
    }
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.get_unsign(key)
    }

    pub fn set_u128(&mut self, key: &str, value: u128) -> &Self {
        self.set_data(key, Type::U128(value))
    }
    pub fn get_u128(&self, key: &str) -> Option<u128> {
        self.get_unsign(key)
    }

    pub fn set_usize(&mut self, key: &str, value: usize) -> &Self {
        self.set_data(key, Type::USIZE(value))
    }
    pub fn get_usize(&self, key: &str) -> Option<usize> {
        self.get_unsign(key)
    }

    pub fn set_f32(&mut self, key: &str, value: f32) -> &Self {
        self.set_data(key, Type::F32(value))
    }
    pub fn get_f32(&self, key: &str) -> Option<f32> {
        if let Some(value) = self.get_data(key) {
            return match *value {
                Type::F32(v) => Some(v as f32),
                Type::F64(v) => Some(v as f32),
                _ => None,
            };
        }
        None
    }

    pub fn set_f64(&mut self, key: &str, value: f64) -> &Self {
        self.set_data(key, Type::F64(value))
    }
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        if let Some(value) = self.get_data(key) {
            return match *value {
                Type::F32(v) => Some(v as f64),
                Type::F64(v) => Some(v as f64),
                _ => None,
            };
        }
        None
    }

    pub fn set_bool(&mut self, key: &str, value: bool) -> &Self {
        self.set_data(key, Type::Boolean(value))
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        if let Some(Type::Boolean(value)) = self.get_data(key) {
            return Some(*value);
        }
        None
    }

    pub fn set_str(&mut self, key: &str, value: &str) -> &Self {
        self.set_data(key, Type::String(value.to_string()))
    }
    pub fn get_str(&self, key: &str) -> Option<&str> {
        if let Some(Type::String(value)) = self.get_data(key) {
            return Some(value.as_str());
        }
        None
    }

    pub fn set_null(&mut self, key: &str) -> &Self {
        self.set_data(key, Type::Null)
    }
    pub fn get_null(&self, key: &str) -> bool {
        if let Some(Type::Null) = self.get_data(key) {
            return true;
        }
        false
    }

    pub fn set_list(&mut self, key: &str, value: JsonList) -> &Self {
        self.set_data(key, Type::JsonList(value))
    }
    pub fn get_list(&self, key: &str) -> Option<&Type> {
        self.get_data(key)
    }

    pub fn set_object(&mut self, key: &str, value: JsonObject) -> &Self {
        self.set_data(key, Type::JsonObject(value))
    }
    pub fn get_object(&self, key: &str) -> Option<&JsonObject> {
        self.get_data(key).and_then(|t| match t {
            Type::JsonObject(obj) => Some(obj),
            _ => None,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }
}

mod string_from_object {
    use super::*;
    use std::fmt::Display;

    impl Display for JsonObject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.to_json())
        }
    }

    impl Debug for JsonObject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("JsonObject {}", self.to_json()))
        }
    }

    impl JsonObject {
        pub fn to_json(&self) -> String {
            let mut result = String::new();
            for pair in &self.list {
                result.push_str(&format!("{},", pair.to_json()))
            }
            if result.len() > 0 {
                result.pop();
            }
            format!("{{{result}}}")
        }

        pub fn to_json_line(&self) -> String {
            self.to_json_line_with_indent(0)
        }

        pub(crate) fn to_json_line_with_indent(&self, indent: usize) -> String {
            let mut result = String::new();
            let start_space = "    ".repeat(indent);
            let tab_space = "    ".repeat(indent + 1);
            let len = &self.list.len();
            let mut index = 0;
            for pair in &self.list {
                if index == len - 1 {
                    // 最后一个元素
                    result.push_str(&format!("{}{}", tab_space, pair.to_json_line(indent + 1)))
                } else {
                    result.push_str(&format!(
                        "{}{},\n",
                        tab_space,
                        pair.to_json_line(indent + 1)
                    ))
                }
                index += 1;
            }
            format!("{{\n{result}\n{start_space}}}")
        }
    }
}

pub(crate) mod obect_from_string {
    use std::str::FromStr;

    use crate::types::Wrap;

    use super::*;
    impl TryFrom<Wrap<&str>> for JsonObject {
        type Error = crate::error::Error;

        fn try_from(value: Wrap<&str>) -> Result<Self, Self::Error> {
            let mut end_ref = Wrap(0);
            let object = parse(&value.0.to_string(), 0, &mut end_ref);
            if object.is_some() {
                Ok(object.unwrap())
            } else {
                Err(crate::error::Error::InvalidJson)
            }
        }
    }

    impl FromStr for JsonObject {
        type Err = crate::error::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut end_ref = Wrap(0);
            let object = parse(&s.to_string(), 0, &mut end_ref);
            if object.is_some() {
                Ok(object.unwrap())
            } else {
                Err(crate::error::Error::InvalidJson)
            }
        }
    }

    pub fn parse(json: &String, start: isize, end_ref: &mut Wrap<isize>) -> Option<JsonObject> {
        if json.is_empty() {
            end_ref.0 = start;
            return None;
        }

        // 找到开始符号
        let mut start_index = 0;
        for i in start..json.len() as isize {
            start_index = i;
            let c = json.char_at(i);
            if c == ' ' || c == '\t' {
                continue;
            }
            if c == '\n' || c == '\r' {
                if c == '\n' && i > 0 && json.char_at(i - 1) == '\r' {
                    //\r\n为一个换行符
                    continue;
                }
                continue;
            }

            if c == '{' {
                // 找到了开始符号
                break;
            }
            return None;
        }
        let start = start_index; // 从开始符号这里开始解析

        let mut param = JsonObject::new();
        // let mut lineNum = 0;

        let mut quote_beg = -1;
        let mut word_beg = -1;

        let mut key = None;

        let mut pair_started = true;
        let mut double_quote = true;

        let i = start;
        if json.char_at(i) != '{' {
            end_ref.0 = i;
            return None;
        }
        let end = json.len() as isize;

        let start = start + 1;
        let mut index = 0;
        // let mut i = 0;
        for i in start..end {
            if i <= index {
                continue;
            }
            let c = json.char_at(i);
            // println!("i={}; index={}; char= {}", i, index, c);
            if c == '\'' || c == '\"' {
                if !pair_started {
                    end_ref.0 = i;
                    return None;
                }
                if quote_beg == -1 {
                    if word_beg != -1 {
                        // println!("wordBeg = {}", wordBeg);
                        end_ref.0 = i;
                        return None;
                    }
                    double_quote = c == '\"';
                    quote_beg = i;
                    word_beg = i + 1;
                } else {
                    // 在引号内
                    if double_quote {
                        // 双引号内
                        if c == '\'' {
                            continue;
                        }
                    } else {
                        if c == '"' {
                            continue;
                        }
                    }
                    if key.is_some() {
                        end_ref.0 = i;
                        return None;
                    }
                    key = var::decode_json(json.substring(word_beg, i));
                    word_beg = -1;
                    quote_beg = -1;
                    // println!("key = {}", key.clone().unwrap());
                }
                continue;
            }

            if quote_beg != -1 {
                // 在引号内
                if c == '\\' {
                    // 转义符i+1，跳过一个字符
                    // i += 1;
                    index = i + 1;
                    // println!("i = {}", i);
                    continue;
                }
                continue;
            }

            if c == ' ' || c == '\n' || c == '\r' || c == '\t' || c == '}' {
                if word_beg != -1 {
                    // 表示单词结束
                    if !pair_started || key.is_some() {
                        end_ref.0 = i;
                        return None;
                    }
                    key = var::decode_json(json.substring(word_beg, i));
                    if key.is_some() && key.clone().unwrap().is_empty() {
                        end_ref.0 = i;
                        return None;
                    }
                    word_beg = -1;
                }

                if c == '}' {
                    if param.is_empty() {
                        pair_started = false; // {}的情况
                    }
                    end_ref.0 = i;
                    // println!("i = {} c = {} pair_started={}", i, c, pair_started);
                    break;
                }
                continue;
            }
            if c == ':' {
                if !pair_started {
                    end_ref.0 = i;
                    return None;
                }
                if key.is_none() {
                    if word_beg == -1 {
                        end_ref.0 = i;
                        return None;
                    }
                    key = var::decode_json(json.substring(word_beg, i));
                    if key.is_some() && key.clone().unwrap().is_empty() {
                        end_ref.0 = i;
                        return None;
                    }
                    word_beg = -1;
                }

                let mut value_end_ref = Wrap(i);
                let value = var::parse_js_value(json.clone(), i + 1, &mut value_end_ref);
                if value.is_none() {
                    end_ref.0 = i;
                    return None;
                }
                // println!("{}={:?}; index={}; i={}", key.clone().unwrap(), value.clone().unwrap(), value_end_ref.0, i);
                param.set_data(&key.unwrap(), value.unwrap());
                key = None; // 重置key
                index = value_end_ref.0;
                pair_started = false;
                continue;
            }

            if c == ',' {
                if key.is_some() || word_beg != -1 {
                    // println!("key = {:?}, wordBeg = {} c={}", key.clone(), wordBeg, &json.clone().charAt(i));
                    end_ref.0 = i;
                    return None;
                }
                pair_started = true;
                // println!("c=, i={i} true==={param:?}");
                continue;
            }
            if word_beg == -1 {
                word_beg = i;
            }
        }

        // 循环结束
        if quote_beg != -1 || key.is_some() || word_beg != -1 {
            return None;
        }

        if pair_started {
            return None;
        }

        Some(param)
    }
}
