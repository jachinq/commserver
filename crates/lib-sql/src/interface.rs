use crate::sources::Dao;

use crate::traits::{CommInterface, Error, Table};
use crate::utils::Search;
use lib_json::object::JsonObject;
use lib_json::types::*;
use sqlite::{Connection, State, Statement};

// 为 sqlite 实现通用接口
impl CommInterface for Dao<Connection> {
    fn list<T: Table>(&self, search: &mut Search) -> Result<Vec<T>, Error> {
        let table_name = T::table_name();

        // 查询总数
        if search.is_seach_total() {
            let mut count = search.clone();
            count.start = 0;
            count.limit = 1;
            count.fileds = "count(*) as cnt".to_string();
            let count = count.parse(table_name);
            println!("count sql={}", count);
            let statement = self.connect.prepare(&count);
            if statement.is_err() {
                if let Err(e) = statement {
                    return Err(Error::SqlError(format!("sql error:{};{:?}", count, e)));
                }
            }
            let mut statement = statement.unwrap();
            while let Ok(State::Row) = statement.next() {
                let cnt = statement.read::<i64, _>("cnt").unwrap();
                search.total = cnt;
                break;
            }
        }

        // 查列表数据
        let mut query = format!("SELECT * FROM {}", table_name);
        let condition = search.parse(table_name);
        if !condition.is_empty() {
            query = condition;
            println!("query={}", query);
        }

        let statement = self.connect.prepare(&query);
        if let Err(e) = statement {
            return Err(Error::SqlError(format!("sql error:{};{:?}", query, e)));
        }
        let mut statement = statement.unwrap();
        read_sqlite_row(&mut statement)
    }

    fn list_all<T: Table>(&self) -> Result<Vec<T>, Error> {
        let query = format!("SELECT * FROM {}", T::table_name());
        let statement = self.connect.prepare(&query);
        // statement.bind((1, 50)).unwrap();
        if let Err(e) = statement {
            return Err(Error::SqlError(format!("sql error:{};{:?}", query, e)));
        }
        let mut statement = statement.unwrap();
        read_sqlite_row(&mut statement)
    }

    fn set<T: Table>(&self, entity: T) -> Result<usize, Error> {
        let mut update = vec![];
        let object = entity.to_json_object().unwrap();
        for column in T::columns() {
            if let Some(v) = object.get_data(column) {
                match v {
                    Type::String(v) => update.push(format!("{}='{}'", column, v)),
                    Type::Boolean(v) => update.push(format!("{}={}", column, v)),
                    Type::JsonObject(v) => update.push(format!("{}='{}'", column, v.to_json())),
                    Type::JsonList(v) => update.push(format!("{}='{}'", column, v.to_json())),
                    Type::Null => update.push(format!("{}=NULL", column)),
                    _ => update.push(format!("{}={}", column, v)),
                }
            }
        }

        if update.is_empty() {
            return Ok(0);
        }
        let sql = format!(
            "update {} set {} where {}={}",
            T::table_name(),
            update.join(","),
            T::id(),
            entity.id_value()
        );
        println!("set sql={}", sql);
        match self.connect.execute(&sql) {
            Ok(_) => Ok(self.connect.change_count()),
            Err(e) => Err(Error::SqlError(format!("{};{}", sql, e))),
        }
    }

    fn delete<T: Table>(&self, entity: T) -> Result<usize, Error> {
        let sql = format!(
            "delete from {} where {}={}",
            T::table_name(),
            T::id(),
            entity.id_value()
        );
        println!("del sql={}", sql);
        match self.connect.execute(&sql) {
            Ok(_) => Ok(self.connect.change_count()),
            Err(e) => Err(Error::SqlError(format!("{};{}", sql, e))),
        }
    }

    fn add<T: Table>(&self, entity: T) -> Result<usize, Error> {
        let mut columns = vec![];
        let mut values = vec![];
        let object = entity.to_json_object().unwrap();
        for column in T::columns() {
            if column == T::id() && T::id_auto_increase() { // 需要自增的id字段不处理
                continue;
            }
            if let Some(v) = object.get_data(column) {
                match v {
                    _ => {
                        columns.push(format!("`{}`", column));
                        values.push(format!("{}", v.to_json()));
                    }
                }
            }
        }

        if columns.is_empty() {
            println!("no columns found");
            return Ok(0);
        }
        let sql = format!(
            "insert into {} ({}) values ({})",
            T::table_name(),
            columns.join(","),
            values.join(",")
        );
        println!("add sql={}，", sql);
        match self.connect.execute(&sql) {
            Ok(_) => Ok(self.connect.change_count()),
            Err(e) => Err(Error::SqlError(format!("{};{}", sql, e))),
        }
    }
}

// 为 sqlite 实现专门接口
impl Dao<Connection> {
    pub fn create_table(&self, sql: &str) -> Result<(), Error> {
        match self.connect.execute(sql) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::SqlError(e.to_string())),
        }
    }
}

// 读取行数据，转换为目标类型的数组返回
fn read_sqlite_row<T: Table>(statement: &mut Statement<'_>) -> Result<Vec<T>, Error> {
    let mut list = vec![];
    while let Ok(State::Row) = statement.next() {
        let mut param = JsonObject::new();
        for column in statement.column_names().iter() {
            let key: &str = &column;
            let ctype = statement.column_type(key).unwrap();
            match ctype {
                sqlite::Type::Float => {
                    let value = statement.read::<f64, _>(key).unwrap();
                    param.set_f64(key, value);
                }
                sqlite::Type::Integer => {
                    let value = statement.read::<i64, _>(key).unwrap();
                    param.set_i64(key, value);
                }
                sqlite::Type::String => {
                    let value = statement.read::<String, _>(key).unwrap();
                    param.set_str(key, &value);
                }
                _ => {
                    param.set_null(key);
                }
            }
        }
        match T::from_json_object(&param) {
            Ok(v) => list.push(v),
            Err(e) => return Err(e),
        }
    }
    Ok(list)
}
