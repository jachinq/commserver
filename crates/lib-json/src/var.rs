use crate::{
    list::JsonList,
    object::obect_from_string,
    types::{Type, Wrap},
};

/// 解析字符串为 json，根据字符匹配分别处理 string,object,list 及其他基础类型
pub fn parse_js_value(json: String, start: isize, end_ref: &mut Wrap<isize>) -> Option<Type> {
    // let mut i = start;
    let mut double_quote = true; // 双引号还是单引号
    let mut quote_beg = -1;
    let mut word_beg = -1;

    let mut index = -1;
    let end = json.len() as isize;
    for i in start..end {
        if i <= index {
            continue;
        }
        // println!("char={c} index={index} i={i}", c=json.char_at(i), index=index, i=i);
        let c = json.char_at(i);
        if c == '\'' || c == '\"' {
            // 字符串的分隔符
            if quote_beg == -1 {
                if word_beg != -1 {
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
                let value = json.substring(word_beg, i); // it's string
                end_ref.0 = i;
                let a = decode_json(value);
                if a.is_none() {
                    return None;
                }
                return Some(Type::String(a.unwrap()));
            }
            continue;
        }
        if quote_beg != -1 {
            // endRef.0 = i;
            // 在引号内
            if c == '\\' {
                // 转义符i+1，跳过一个字符
                index = i + 1;
                continue;
            }
            continue;
        }

        if c == '{' {
            // param的分隔符
            if word_beg != -1 {
                // 此前不应该有单词开始
                end_ref.0 = i;
                return None;
            }
            let mut value_end_ref = Wrap(0);
            let param = obect_from_string::parse(&json, start, &mut value_end_ref);
            // println!("param = {:?} start={} index={}", param, start, value_end_ref.0);
            index = value_end_ref.0;
            end_ref.0 = index;
            if param.is_none() {
                return None;
            }
            return Some(Type::JsonObject(param.unwrap()));
        }

        if c == '[' {
            // list的分隔符
            if word_beg != -1 {
                // 此前不应该有单词开始
                end_ref.0 = i;
                return None;
            }
            let mut value_end_ref = Wrap(i);
            let value = JsonList::parse_list(json.clone(), start, &mut value_end_ref);
            end_ref.0 = value_end_ref.0;
            // println!("value===={:?}", value);
            if value.is_none() {
                return None;
            }
            return Some(Type::JsonList(value.unwrap()));
        }

        // 其它类型的分隔符
        if c == ' ' || c == '\n' || c == '\r' || c == '\t' || c == ',' || c == '}' || c == ']' {
            if word_beg == -1 {
                if c == ',' || c == '}' {
                    // 这几个分隔符必须有单词开始了
                    end_ref.0 = i;
                    return None;
                }
                continue;
            }
            let value = json.substring(word_beg, i);

            if c == ',' || c == '}' || c == ']' {
                // 因为这些都是特殊分隔符，所以这里要回退一个作为end
                index = i - 1; // todo
                end_ref.0 = index;
                // println!("char={c} index={index} i={i}")
            }
            return Type::parse_type(&value);
        }
        if word_beg == -1 {
            word_beg = i;
        }
        end_ref.0 = i;
    }

    return None;
}

/// 解析字符串为 json 的基础数字类型：number, boolean, null, nan
pub fn parse_js_value_number(value: String) -> Option<Type> {
    if value.equals_ignore_case("true") {
        return Some(Type::Boolean(true));
    } else if value.equals_ignore_case("false") {
        return Some(Type::Boolean(false));
    } else if value.index_of(".").is_some() {
        return Some(Type::F64(value.parse().unwrap()));
    } else if value.equals_ignore_case("null") || value.equals_ignore_case("nan") {
        return None;
    } else {
        let a = value.parse();
        if a.is_err() {
            println!("parse_js_value_number error: {}", value);
            return None;
        }
        return Some(Type::ISIZE(a.unwrap()));
    }
}

pub fn decode_json(js: String) -> Option<String> {
    if js.is_empty() {
        return Some("".to_string());
    }

    let mut buf = String::new();
    let mut in_trans = false; // string 是否处于转义中
    let mut count = 0;

    for (i, c) in js.chars().enumerate() {
        if count != 0 {
            count -= 1;
            continue;
        }

        if in_trans {
            match c {
                '"' | '\'' | '/' | '\\' => buf.push_str(c.to_string().as_str()),
                'b' => buf.push_str(r"\b"),
                'f' => buf.push_str(r"\f"),
                'n' => buf.push_str(r"\n"),
                'r' => buf.push_str(r"\r"),
                't' => buf.push_str(r"\t"),
                'u' => {
                    // unicode 解码
                    let start_index = (i + 1) as isize;
                    let next_index = (i + 1 + 4) as isize;
                    if next_index <= js.len() as isize {
                        count = 4;
                        let result = unicode_to_char(&js.substring(start_index, next_index));
                        if result.is_some() {
                            buf.push(result.unwrap());
                        }
                    }
                }
                _ => (),
            }

            in_trans = false;
        } else if c == '\\' {
            in_trans = true;
        } else {
            in_trans = false;
            buf.push_str(c.to_string().as_str());
        }
    }
    return Some(buf);
}

// unicode to char
fn unicode_to_char(unicode: &str) -> Option<char> {
    // 去除前面的转义字符，提取出Unicode码点
    let unicode = unicode.trim_start_matches("\\u").to_uppercase();
    let unicode_code = unicode.trim_start_matches("\\U").to_uppercase();
    // 将16进制字符串解析为u32
    match u32::from_str_radix(&unicode_code, 16) {
        Ok(code) => char::from_u32(code), // 将u32转换为char
        Err(_) => None,                   // 如果解析失败，则返回None
    }
}

pub trait StringExt {
    fn substring(&self, start: isize, end: isize) -> String;
    fn char_at(&self, index: isize) -> char;
    fn equals_ignore_case(&self, other: &str) -> bool;
    fn index_of(&self, other: &str) -> Option<isize>;
}
impl StringExt for String {
    fn substring(&self, start: isize, end: isize) -> String {
        let mut s = String::new();
        let json = self.clone();
        for i in start..end {
            s.push(json.chars().nth(i as usize).unwrap());
        }
        s
    }

    fn char_at(&self, index: isize) -> char {
        self.chars().nth(index as usize).unwrap()
    }

    fn equals_ignore_case(&self, other: &str) -> bool {
        self.to_lowercase() == other.to_lowercase()
    }

    fn index_of(&self, other: &str) -> Option<isize> {
        let mut i = 0;
        for c in self.chars() {
            if c == other.chars().nth(0).unwrap() {
                if self.chars().skip(i).take(other.len()).collect::<String>() == other {
                    return Some(i as isize);
                }
            }
            i += 1;
        }
        None
    }
}
