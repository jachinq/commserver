use lib_json::object::JsonObject;
use lib_sql::{
    traits::{Error, Table},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonResult<T: Serialize> {
    code: i32,
    success: bool,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> JsonResult<T> {
    pub fn new(code: i32, success: bool, message: &str, data: Option<T>) -> Self {
        Self {
            code,
            success,
            message: message.to_string(),
            data,
        }
    }
    pub fn success(data: T) -> Self {
        Self::new(0, true, "success", Some(data))
    }

    pub fn error(message: &str) -> Self {
        Self::new(-1, false, message, None)
    }
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self::new(0, true, "success", None)
    }
}

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
