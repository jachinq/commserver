use lib_json::types::Type;
use lib_sql::{
    SqliteConnection,
    sources::Dao,
    traits::CommInterface,
    utils::{Operator, Search},
};
use crate::{model::User, utils::auth::init_user};

pub fn init() -> Result<(), lib_sql::traits::Error> {
    let dao = Dao::new()?;
    init_table(&dao)?;
    init_data(&dao)?;
    Ok(())
}


// 定义 init_table 函数，用于初始化数据库表
fn init_table(dao: &Dao<SqliteConnection>) -> Result<(), lib_sql::traits::Error> {
    dao.create_table(&init_table_user())?;
    Ok(())
}

// 定义 init_data 函数，用于初始化默认的用户
fn init_data(dao: &Dao<SqliteConnection>) -> Result<(), lib_sql::traits::Error> {
    let user = init_user();
    let mut search = Search::default();
    search.matcher.and(
        "username",
        Operator::Eq,
        Type::String(user.username.to_string()),
    );
    let users = dao.list::<User>(&mut search)?;
    if users.len() > 0 {
        return Ok(());
    }

    dao.add(user)?;
    Ok(())
}


// 创建用户表
fn init_table_user() -> String {
    r#"CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            email TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )"#
    .to_string()
}