#![allow(dead_code)]

use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, Timelike};

use crate::convert::{date_to_timestamp, datetime_to_timestamp, timestamp_to_date};

pub struct Calendar {
    now: DateTime<FixedOffset>,
}

impl Calendar {
    pub fn now() -> Self {
        Self {
            now: Local::now().fixed_offset(),
        }
    }

    pub fn year(&self) -> i32 {
        self.now.year()
    }

    pub fn month(&self) -> u32 {
        self.now.month()
    }

    pub fn day(&self) -> u32 {
        self.now.day()
    }

    pub fn hour(&self) -> u32 {
        self.now.hour()
    }

    pub fn minute(&self) -> u32 {
        self.now.minute()
    }

    pub fn second(&self) -> u32 {
        self.now.second()
    }

    pub fn week_beg(&self) -> Self {
        let week_beg = self.now - Duration::days(self.now.weekday().num_days_from_monday() as i64);
        Self::from_datetime(&week_beg).days_beg(0)
    }
    pub fn week_end(&self) -> Self {
        let week_end =
            self.now + Duration::days(6 - self.now.weekday().num_days_from_monday() as i64);
        Self::from_datetime(&week_end).days_end(0)
    }
    pub fn month_beg(&self) -> Self {
        let month_beg = self.now.with_day(1).unwrap();
        Self::from_datetime(&month_beg).days_beg(0)
    }
    pub fn month_end(&self) -> Self {
        let month_end = self
            .now
            .with_day(self.now.num_days_in_month() as u32)
            .unwrap();
        Self::from_datetime(&month_end).days_end(0)
    }
    pub fn year_beg(&self) -> Self {
        let year_beg = self.now.with_month(1).unwrap();
        Self::from_datetime(&year_beg).month_beg()
    }
    pub fn year_end(&self) -> Self {
        let year_end = self.now.with_month(12).unwrap();
        Self::from_datetime(&year_end).month_end()
    }

    pub fn days_beg(&self, days: i32) -> Self {
        let start_date = self.now + Duration::days(days as i64);
        let start_date = timestamp_to_date(start_date.timestamp());
        let date_beg = date_to_timestamp(&start_date);
        Self::from_timestamp(date_beg)
    }

    pub fn days_end(&self, days: i32) -> Self {
        let start_date = self.now + Duration::days(days as i64);
        let start_date = timestamp_to_date(start_date.timestamp());
        let date_end = datetime_to_timestamp(&format!("{} 23:59:59", start_date));
        Self::from_timestamp(date_end)
    }

    pub fn months_beg(&self, month: i32) -> Self {
        let target_date = if month >= 0 {
            self.now + Duration::days(month as i64 * 30) // 近似计算
        } else {
            // 对于负数月份，需要准确计算
            let mut year = self.now.year();
            let mut target_month = self.now.month() as i32 + month;

            while target_month <= 0 {
                target_month += 12;
                year -= 1;
            }
            while target_month > 12 {
                target_month -= 12;
                year += 1;
            }

            self.now
                .with_year(year)
                .unwrap()
                .with_month(target_month as u32)
                .unwrap()
                .with_day(1)
                .unwrap()
        };

        let target_date = if month >= 0 {
            target_date.with_day(1).unwrap()
        } else {
            target_date
        };

        Self::from_datetime(&target_date).days_beg(0)
    }
    pub fn months_end(&self, month: i32) -> Self {
        let target_date = if month >= 0 {
            self.now + Duration::days(month as i64 * 30) // 近似计算
        } else {
            // 对于负数月份，需要准确计算
            let mut year = self.now.year();
            let mut target_month = self.now.month() as i32 + month;

            while target_month <= 0 {
                target_month += 12;
                year -= 1;
            }
            while target_month > 12 {
                target_month -= 12;
                year += 1;
            }

            self.now
                .with_year(year)
                .unwrap()
                .with_month(target_month as u32)
                .unwrap()
        };

        let target_date = if month >= 0 {
            let days_in_month = target_date.num_days_in_month() as u32;
            target_date.with_day(days_in_month).unwrap()
        } else {
            let days_in_month = target_date.num_days_in_month() as u32;
            target_date.with_day(days_in_month).unwrap()
        };

        Self::from_datetime(&target_date).days_end(0)
    }
    pub fn years_beg(&self, year: i32) -> Self {
        let target_year = self.now.year() + year;
        let target_date = self
            .now
            .with_year(target_year)
            .unwrap()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap();
        Self::from_datetime(&target_date).days_beg(0)
    }
    pub fn years_end(&self, year: i32) -> Self {
        let target_year = self.now.year() + year;
        let target_date = self
            .now
            .with_year(target_year)
            .unwrap()
            .with_month(12)
            .unwrap()
            .with_day(31)
            .unwrap();
        Self::from_datetime(&target_date).days_end(0)
    }

    pub fn from_timestamp(timestamp: i64) -> Self {
        let native = match DateTime::from_timestamp(timestamp, 0) {
            Some(v) => v,
            None => {
                println!("Invalid timestamp, {}", timestamp);
                return Self::now();
            }
        };
        Self {
            now: native.with_timezone(Local::now().offset()),
        }
    }

    /// 从字符串中获取时间，兼容两种格式：YYYY-MM-DD HH:MM:SS 和 YYYY-MM-DD
    pub fn from_datetime(date: &DateTime<FixedOffset>) -> Self {
        Self { now: *date }
    }

    /// 从字符串中获取时间，兼容两种格式：YYYY-MM-DD HH:MM:SS 和 YYYY-MM-DD
    pub fn from_datetime_str(datetime: &str) -> Self {
        if datetime.contains(" ") {
            Self::from_timestamp(datetime_to_timestamp(datetime))
        } else {
            Self::from_timestamp(date_to_timestamp(datetime))
        }
    }

    pub fn format_datetime(&self) -> String {
        self.now.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn format_date(&self) -> String {
        self.now.format("%Y-%m-%d").to_string()
    }

    pub fn format_time(&self) -> String {
        self.now.format("%H:%M:%S").to_string()
    }

    pub fn timestamp(&self) -> i64 {
        self.now.timestamp()
    }
}

#[test]
fn test_now() {
    let now = Calendar::now();
    let local = Local::now();
    assert_eq!(now.year(), local.year());
    assert_eq!(now.month(), local.month());
    assert_eq!(now.day(), local.day());
    assert_eq!(now.hour(), local.hour());
    assert_eq!(now.minute(), local.minute());
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateRange {
    Custom(String, String),
    CurrentWeek,
    CurrentMonth,
    CurrentYear,
    PreviousWeek,
    PreviousMonth,
    PreviousYear,
    Last7Days,
    Last14Days,
    Last30Days,
}

impl From<&str> for DateRange {
    fn from(value: &str) -> Self {
        match value {
            "1" => DateRange::CurrentWeek,
            "2" => DateRange::CurrentMonth,
            "3" => DateRange::CurrentYear,
            "4" => DateRange::PreviousWeek,
            "5" => DateRange::PreviousMonth,
            "6" => DateRange::PreviousYear,
            "7" => DateRange::Last7Days,
            "8" => DateRange::Last14Days,
            "9" => DateRange::Last30Days,
            _ => DateRange::Custom(String::new(), String::new()),
        }
    }
}

impl DateRange {
    pub fn get_range(&self) -> (String, String) {
        match self {
            DateRange::CurrentWeek => (
                format!("{}", Calendar::now().week_beg().format_datetime()),
                format!("{}", Calendar::now().week_end().format_datetime()),
            ),
            DateRange::CurrentMonth => (
                format!("{}", Calendar::now().month_beg().format_datetime()),
                format!("{}", Calendar::now().month_end().format_datetime()),
            ),
            DateRange::CurrentYear => (
                format!("{}", Calendar::now().year_beg().format_datetime()),
                format!("{}", Calendar::now().year_end().format_datetime()),
            ),
            DateRange::PreviousWeek => (
                format!(
                    "{}",
                    Calendar::now().days_beg(-7).week_beg().format_datetime()
                ),
                format!(
                    "{}",
                    Calendar::now().days_end(-7).week_end().format_datetime()
                ),
            ),
            DateRange::PreviousMonth => (
                format!("{}", Calendar::now().months_beg(-1).format_datetime()),
                format!("{}", Calendar::now().months_end(-1).format_datetime()),
            ),
            DateRange::PreviousYear => (
                format!("{}", Calendar::now().years_beg(-1).format_datetime()),
                format!("{}", Calendar::now().years_end(-1).format_datetime()),
            ),
            DateRange::Last7Days => (
                format!("{}", Calendar::now().days_beg(-7).format_datetime()),
                format!("{}", Calendar::now().days_end(0).format_datetime()),
            ),
            DateRange::Last14Days => (
                format!("{}", Calendar::now().days_beg(-14).format_datetime()),
                format!("{}", Calendar::now().days_end(0).format_datetime()),
            ),
            DateRange::Last30Days => (
                format!("{}", Calendar::now().days_beg(-30).format_datetime()),
                format!("{}", Calendar::now().days_end(0).format_datetime()),
            ),
            DateRange::Custom(start, end) => (start.to_string(), end.to_string()),
        }
    }
}

#[test]
fn test_date_range() {
    assert_eq!(
        DateRange::CurrentWeek.get_range(),
        (
            "2025-09-15 00:00:00".to_string(),
            "2025-09-21 23:59:59".to_string()
        )
    );

    assert_eq!(
        DateRange::CurrentMonth.get_range(),
        (
            "2025-09-01 00:00:00".to_string(),
            "2025-09-30 23:59:59".to_string()
        )
    );

    assert_eq!(
        DateRange::CurrentYear.get_range(),
        (
            "2025-01-01 00:00:00".to_string(),
            "2025-12-31 23:59:59".to_string()
        )
    );

    assert_eq!(
        DateRange::PreviousWeek.get_range(),
        (
            "2025-09-08 00:00:00".to_string(),
            "2025-09-14 23:59:59".to_string()
        )
    );

    assert_eq!(
        DateRange::PreviousMonth.get_range(),
        (
            "2025-08-01 00:00:00".to_string(),
            "2025-08-31 23:59:59".to_string()
        )
    );

    assert_eq!(
        DateRange::PreviousYear.get_range(),
        (
            "2024-01-01 00:00:00".to_string(),
            "2024-12-31 23:59:59".to_string()
        )
    );

    assert_eq!(
        DateRange::Last7Days.get_range(),
        (
            "2025-09-12 00:00:00".to_string(),
            "2025-09-19 23:59:59".to_string()
        )
    );

    assert_eq!(
        DateRange::Last14Days.get_range(),
        (
            "2025-09-05 00:00:00".to_string(),
            "2025-09-19 23:59:59".to_string()
        )
    );

    assert_eq!(
        DateRange::Last30Days.get_range(),
        (
            "2025-08-20 00:00:00".to_string(),
            "2025-09-19 23:59:59".to_string()
        )
    );

    assert_eq!(
        DateRange::Custom(
            "2025-09-01 00:00:00".to_string(),
            "2025-09-30 23:59:59".to_string()
        )
        .get_range(),
        (
            "2025-09-01 00:00:00".to_string(),
            "2025-09-30 23:59:59".to_string()
        )
    )
}
