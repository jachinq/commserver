//! # lib-sql
//!
//! 一个用于数据库操作的 orm 库。支持 sqlite 和 mysql

pub mod interface;
pub mod sources;
pub mod traits;
pub mod utils;

pub use sqlite::Connection as SqliteConnection;