#![allow(dead_code)]
use chrono::{DateTime, Local};

/// 获取本地时间时间戳
pub fn local_timestamp() -> i64 {
    Local::now().timestamp()
}

/// 获取本地时间字符串，使用本地时区
pub fn local_datetime() -> String {
    timestamp_to_datetime(local_timestamp())
}

/// 从 i64 时间戳转换为字符串，使用本地时区
pub fn timestamp_to_string(timestamp: i64, format: &str) -> String {
    let native = match DateTime::from_timestamp(timestamp, 0) {
        Some(v) => v,
        None => return "".to_string(),
    };
    let dt = native.with_timezone(Local::now().offset());
    dt.format(format).to_string()
}

/// 从 i64 时间戳转换为字符串 YYYY-MM-DD HH:MM:SS，使用本地时区
pub fn timestamp_to_datetime(timestamp: i64) -> String {
    timestamp_to_string(timestamp, "%Y-%m-%d %H:%M:%S")
}

/// 从 i64 时间戳转换为字符串 YYYY-MM-DD，使用本地时区
pub fn timestamp_to_date(timestamp: i64) -> String {
    timestamp_to_string(timestamp, "%Y-%m-%d")
}


/// 从 i64 时间戳转换为字符串 HH:MM:SS，使用本地时区
pub fn timestamp_to_time(timestamp: i64) -> String {
    timestamp_to_string(timestamp, "%H:%M:%S")
}


/// 从字符串转换为 i64 时间戳，使用本地时区，谨慎使用，字符串格式和 format 参数必须严格一致，format 格式：%Y-%m-%d %H:%M:%S %z
pub fn string_to_timestamp(s: &str, format: &str) -> i64 {
    let dt = match DateTime::parse_and_remainder(s, format) {
        Ok(v) => v,
        Err(e) => {
            println!("parse_from_str error: {}", e);
            return 0;
        }
    }
    .0;
    let dt = dt.with_timezone(Local::now().offset());
    dt.timestamp()
}

// 从字符串转换为 i64 时间戳，使用本地时区，字符串格式：YYYY-MM-DD HH:MM:SS
pub fn datetime_to_timestamp(s: &str) -> i64 {
    let s = format!("{} {}", s, Local::now().offset());
    let format = "%Y-%m-%d %H:%M:%S %z"; // 固定时区
    string_to_timestamp(&s, format)
}

/// 从字符串转换为 i64 时间戳，使用本地时区，字符串格式：YYYY-MM-DD
pub fn date_to_timestamp(s: &str) -> i64 {
    datetime_to_timestamp(&format!("{} 00:00:00", s))
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_to_string() {
        let timestamp = 1626211200;
        let format = "%Y-%m-%d %H:%M:%S";
        let s = timestamp_to_string(timestamp, format);
        assert_eq!(s, "2021-07-14 05:20:00");

        let format = "%Y-%m-%d %H:%M:%S %z";
        let s = timestamp_to_string(timestamp, format);
        assert_eq!(s, "2021-07-14 05:20:00 +0800");

        let format = "%Y-%m-%d";
        let s = timestamp_to_string(timestamp, format);
        assert_eq!(s, "2021-07-14");

        let format = "%Y-%m-%d %H:%M";
        let s = timestamp_to_string(timestamp, format);
        assert_eq!(s, "2021-07-14 05:20");

        let s = timestamp_to_datetime(timestamp);
        assert_eq!(s, "2021-07-14 05:20:00");

        let s = timestamp_to_date(timestamp);
        assert_eq!(s, "2021-07-14");

        let s = timestamp_to_time(timestamp);
        assert_eq!(s, "05:20:00");
    }    

    #[test]
    fn test_string_to_timestamp() {
        println!("offset: {}", Local::now().offset());
        let s = format!("{} {}", "2021-07-14 05:20:00", Local::now().offset());
        let format = "%Y-%m-%d %H:%M:%S %z";
        let timestamp = string_to_timestamp(&s, format);
        assert_eq!(timestamp, 1626211200);
    }

    #[test]
    fn test_datetime_to_timestamp() {
        let s = "2021-07-14 05:20:00";
        let timestamp = datetime_to_timestamp(s);
        assert_eq!(timestamp, 1626211200);
    }

    #[test]
    fn test_date_to_timestamp() {
        let s = "2021-07-14";
        let timestamp = date_to_timestamp(s);
        assert_eq!(timestamp, 1626192000);
    }
}
