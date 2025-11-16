use lib_json::object::JsonObject;
use lib_sql::{
    sources::Dao,
    traits::{CommInterface, Error, Table},
    utils::{Matcher, Operator, Search},
};
use serde::{Deserialize, Serialize};

// 定义 User 结构体
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: i32,
}

impl Table for User {
    fn id() -> &'static str {
        "id"
    }

    fn id_auto_increase() -> bool {
        true
    }

    fn columns() -> Vec<&'static str> {
        vec!["id", "username", "password", "email", "created_at"]
    }

    fn id_value(&self) -> String {
        self.id.to_string()
    }

    fn table_name() -> &'static str {
        "user"
    }

    fn from_json_object(row: &JsonObject) -> Result<Self, Error> {
        let mut entity = Self::default();
        match row.get_i32("id") {
            Some(v) => entity.id = v,
            None => return Err(Error::ParseError),
        }
        match row.get_str("username") {
            Some(v) => entity.username = v.to_string(),
            None => return Err(Error::ParseError),
        }
        match row.get_str("password") {
            Some(v) => entity.password = v.to_string(),
            None => return Err(Error::ParseError),
        }
        match row.get_str("email") {
            Some(v) => entity.email = v.to_string(),
            None => return Err(Error::ParseError),
        }
        match row.get_i32("created_at") {
            Some(v) => entity.created_at = v,
            None => return Err(Error::ParseError),
        }
        Ok(entity)
    }

    fn to_json_object(&self) -> Result<JsonObject, Error> {
        let mut obj = JsonObject::new();
        obj.set_i32("id", self.id);
        obj.set_str("username", &self.username);
        obj.set_str("password", &self.password);
        obj.set_str("email", &self.email);
        obj.set_i32("created_at", self.created_at);
        Ok(obj)
    }
    
}

fn main() {
    let dao = Dao::new();
    if let Err(e) = dao {
        println!("{:?}", e);
        return;
    }
    let dao = dao.unwrap();

    // 创建表
    let sql = "CREATE TABLE IF NOT EXISTS user ( id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL, password TEXT NOT NULL, email TEXT NOT NULL, created_at INTEGER NOT NULL );";
    if let Err(e) = dao.create_table(sql) {
        println!("create table error；{:?}", e);
        return;
    } else {
        println!("create table ok");
    }

    if let Ok(list) = dao.list_all::<User>() {
        println!("list size={}", list.len());
        for ele in list {
            println!("{:?}, id={} username={}", ele, ele.id, ele.username);
        }
    };

    let p = User {
        id: 1,
        username: "从app修改".to_string(),
        password: "password".to_string(),
        email: "email".to_string(),
        created_at: 1615299200,
    };
    match dao.set(p) {
        Ok(cnt) => println!("set ok, cnt={}", cnt),
        Err(e) => println!("set err: {:?}", e),
    }
    let mut p = User::default();
    p.id = 1;
    match dao.delete(p) {
        Ok(cnt) => println!("del ok, cnt={}", cnt),
        Err(e) => println!("del err: {:?}", e),
    }
    let p = User {
        id: 1,
        username: "从app新增".to_string(),
        password: "password".to_string(),
        email: "email".to_string(),
        created_at: 1615299200,
    };
    match dao.add(p) {
        Ok(cnt) => println!("add ok, cnt={}", cnt),
        Err(e) => println!("add err: {:?}", e),
    }

    let mut matcher_or1 = Matcher::new();
    matcher_or1.or("id", Operator::Eq, 1);
    let mut matcher_or2 = Matcher::new();
    matcher_or2.or("id", Operator::Eq, 2);
    let mut matcher_and = Matcher::new();
    matcher_and.and("username", Operator::Like, "app");

    let mut search = Search::default();
    search.matcher.and("id", Operator::In, "1,2,3,4");
    search.matcher.and_matcher(matcher_and);
    search.matcher.or_matcher(matcher_or1);
    search.matcher.or_matcher(matcher_or2);
    search.sort.add("id", true);
    search.sort.add("username", false);
    search.start = 0;
    search.limit = 1;
    match dao.list::<User>(&mut search) {
        Ok(list) => println!("list={:?}", list),
        Err(e) => println!("e={:?}", e),
    }
    println!("total = {}", search.total);
}
