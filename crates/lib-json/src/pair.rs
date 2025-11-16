use crate::types::Type;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub struct Pair {
    pub first: String,
    pub second: Type,
}

impl Pair {
    pub fn new(key: &str, value: Type) -> Self {
        Self {
            first: key.to_string(),
            second: value,
        }
    }
}
impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_json())
    }
}

mod string_from_pair {
    use super::*;

    impl Pair {
        pub fn to_json(&self) -> String {
            format!("\"{}\":{}", self.first, &self.second.to_json())
        }
        pub(crate) fn to_json_line(&self, indent: usize) -> String {
            format!("\"{}\": {}", self.first, &self.second.to_json_line(indent))
        }
    }
}
