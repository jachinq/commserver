use lib_json::object::JsonObject;

use crate::utils::Search;

/// 统一错误类
#[derive(Debug)]
pub enum Error {
    ParseError,
    SqlError(String),
    ConfigError(String),
    ArgError(String),
}

/// 为每个表对应的结构体实现该 trait
pub trait Table: Sized {
    /// 表的主键，如 "id"
    fn id() -> &'static str;
    
    /// 表的主键是否自增
    fn id_auto_increase() -> bool;

    /// 表的字段，如:
    /// ```
    /// fn columns() -> Vec<&'static str> {
    ///     vec!["id", "name"]
    /// }
    /// ```
    fn columns() -> Vec<&'static str>;

    /// 表的主键内容，如：
    /// ```
    /// // 表对应的结构体
    /// #[derive(Debug, Default)]
    /// struct Animals {
    ///     id: i32, // 主键字段
    ///     name: String
    /// }
    ///
    /// // 返回主键内容
    /// Animals::default().id.to_string();
    /// ```
    fn id_value(&self) -> String;

    /// 表名，如："animals"
    fn table_name() -> &'static str;

    /// 从 JsonObject 转为 Entity，如：
    /// ```
    /// use lib_json::object::JsonObject;
    /// let row = JsonObject::new();
    /// // 表对应的结构体
    /// #[derive(Debug, Default)]
    /// struct Animals {
    ///     id: i32,
    ///     name: String
    /// }
    ///
    /// let mut entity = Animals::default();
    /// match row.get_i32("id") {
    ///     Some(v) => entity.id = v,
    ///     None => {}, // 返回错误
    /// }
    /// match row.get_str("name") {
    ///     Some(v) => entity.name = v.to_string(),
    ///     None => {},
    /// }
    /// // 返回结构体
    /// // Ok(entity)
    /// ```
    fn from_json_object(v: &JsonObject) -> Result<Self, Error>;

    /// 从 Entity 转为 JsonObject，如：
    /// ```
    ///
    /// use lib_json::object::JsonObject;
    /// use lib_sql::traits::Error;
    ///
    /// // 表对应的结构体
    /// #[derive(Debug, Default)]
    /// struct Animals {
    ///     id: i32,
    ///     name: String
    /// }
    ///
    /// let animal = Animals::default();
    /// let mut obj = JsonObject::new();
    /// obj.set_i32("id", animal.id);
    /// obj.set_str("name", &animal.name);
    /// // 返回 obj
    /// // Ok(obj)
    /// ```
    fn to_json_object(&self) -> Result<JsonObject, Error>;
}

/// 为不同数据库提供通用的接口
/**
 * list: 按指定条件查询
 * list_all: 查询全表
 * set: 修改数据
 *  delete: 删除数据
 *  add: 添加数据
 */
pub trait CommInterface {
    fn list<T: Table>(&self, search_arg: &mut Search) -> Result<Vec<T>, Error>;
    fn list_all<T: Table>(&self) -> Result<Vec<T>, Error>;
    fn set<T: Table>(&self, entity: T) -> Result<usize, Error>;
    fn delete<T: Table>(&self, entity: T) -> Result<usize, Error>;
    fn add<T: Table>(&self, entity: T) -> Result<usize, Error>;
}

/// 区分不同数据库的连接，为每个数据库实现该 trait
pub trait Connect {}
