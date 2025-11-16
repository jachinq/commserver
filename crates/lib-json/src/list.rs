use crate::{
    types::{Type, Wrap},
    var::{self, StringExt},
};

#[derive(Clone, PartialEq)]
pub struct JsonList(pub Vec<Type>);

impl JsonList {
    // 转换为Type
    fn convert_to_type<T: ToString>(v: &T) -> Type {
        let value = Type::parse_type(&v.to_string());
        match value {
            Some(r#type) => r#type,
            None => Type::String(format!("{}", v.to_string())),
        }
    }

    pub fn new<T: ToString>(values: &[T]) -> Self {
        Self(values.iter().map(|v| Self::convert_to_type(v)).collect())
    }
    pub fn from_vec(values: Vec<Type>) -> Self {
        Self(values)
    }

    pub fn push(&mut self, value: Type) {
        self.0.push(value)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn get(&self, index: usize) -> Option<&Type> {
        self.0.get(index)
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Type> {
        self.0.get_mut(index)
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Type> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Type> {
        self.0.iter_mut()
    }
}

mod string_from_list {
    use std::fmt::{Debug, Display};

    use super::JsonList;

    impl Display for JsonList {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{}", self.to_json()))
        }
    }

    impl Debug for JsonList {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("JsonList").field(&self.0).finish()
        }
    }
}

pub(crate) mod list_from_string {
    use std::str::FromStr;

    use super::*;

    /// let list: JsonList = str.parse().unwrap();
    impl FromStr for JsonList {
        type Err = crate::error::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parse = JsonList::parse(s.to_string());
            if parse.is_none() {
                return Err(crate::error::Error::InvalidJson);
            }
            let parse = parse.unwrap();
            Ok(parse)
        }
    }

    impl JsonList {
        pub fn to_json(&self) -> String {
            let mut result = String::new();
            for t in self.0.to_vec() {
                result.push_str(&format!("{},", &t.to_json()));
            }
            if result.len() > 0 {
                result.pop();
            }
            format!("[{}]", result)
        }

        pub fn to_json_line(&self) -> String {
            self.to_json_line_with_indent(0)
        }
        pub(crate) fn to_json_line_with_indent(&self, indent: usize) -> String {
            let mut result = String::new();
            let start_space = "    ".repeat(indent);
            let tab_space = "    ".repeat(indent + 1);
            let len = &self.0.len();
            let mut index = 0;

            for t in &self.0 {
                if index == len - 1 {
                    result.push_str(&format!("{}{}", tab_space, t.to_json_line(indent + 1)));
                } else {
                    result.push_str(&format!("{}{},\n", tab_space, t.to_json_line(indent + 1)));
                }
                index += 1;
            }
            format!("[\n{result}\n{start_space}]")
        }

        pub(crate) fn parse(json: String) -> Option<JsonList> {
            Self::parse_list(json, 0, &mut Wrap(0))
        }

        pub(crate) fn parse_list(
            json: String,
            start: isize,
            end_ref: &mut Wrap<isize>,
        ) -> Option<JsonList> {
            // 找到开始位置
            let mut start_index = 0;
            for i in start..json.len() as isize {
                let c = json.char_at(i);
                if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
                    continue;
                }
                if c == '[' {
                    // 找到开始位置
                    start_index = i;
                    break;
                }
                return None;
            }
            let start = start_index; // 从这里开始解析

            let mut list = vec![];
            let mut value_started = true; // 是否开始一个value
            let c = json.char_at(start);
            if c != '[' {
                return None;
            }
            let mut current_index = 0; // 当前处理到的下标
            let start = start + 1;
            let end = json.len() as isize;
            for i in start..end {
                if i <= current_index {
                    continue;
                }
                let c = json.char_at(i);
                // println!("i={}; index={}; char= {}", i, current_index, c);
                if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
                    continue;
                }

                if c == ']' {
                    if list.is_empty() {
                        value_started = false; // []的情况
                    }
                    current_index = i;
                    break;
                }

                if value_started {
                    let mut value_end_ref = Wrap(i);
                    let value = var::parse_js_value(json.clone(), i, &mut value_end_ref);
                    // println!("value----{value:?} i={i} c={c} next={}", json.charAt(i+1));
                    if value.is_none() {
                        current_index = i;
                        break;
                    }

                    list.push(value.unwrap());
                    current_index = value_end_ref.0;
                    value_started = false;
                    continue;
                }

                if c == ',' {
                    value_started = true;
                    continue;
                }

                break;
            }

            end_ref.0 = current_index;

            // 循环结束
            if value_started {
                return None;
            }
            Some(JsonList::from_vec(list))
        }
    }
}
