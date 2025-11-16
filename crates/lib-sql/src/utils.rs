use serde::Deserialize;

use crate::traits::Error;
use std::fs;

/// 配置文件
#[derive(Debug, Deserialize)]
pub struct Config {
    pub datasource: String,
    pub webdir: Option<String>,
    pub port: Option<u16>,

}

/// 读取指定路径的配置文件，配置文件使用 toml 进行解析
pub fn read_config(path: &str) -> Result<Config, Error> {
    if let Err(e) = fs::exists(path) {
        return Err(Error::ConfigError(format!("path={}, e={:?}", path, e)));
    }
    let file = fs::read(path);
    if let Err(e) = file {
        return Err(Error::ConfigError(format!("path={}, e={:?}", path, e)));
    }
    let file = file.unwrap();
    let config = toml::from_slice(&file);
    if let Err(e) = config {
        return Err(Error::ConfigError(format!("parse config {:?}", e)));
    }
    let config: Config = config.unwrap();
    println!("{:?}", config);
    Ok(config)
}

/// sql 条件操作符，包含 不等于、等于、大于、大于等于、小于、小于等于、包含、不包含、模糊包含、模糊不包含、位运算等于、位运算不等于
#[derive(Debug)]
pub enum Operator {
    Ne,
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    In,
    NotIn,
    Land,
    LandNe,
    Like,
    NotLike,
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Ne => String::from("<>"),
            Operator::Eq => String::from("="),
            Operator::Ge => String::from(">="),
            Operator::Gt => String::from(">"),
            Operator::Le => String::from("<="),
            Operator::Lt => String::from("<"),
            Operator::In => String::from("in"),
            Operator::NotIn => String::from("not in"),
            Operator::Land => String::from("&"),
            Operator::LandNe => String::from("&<>"),
            Operator::Like => String::from("like"),
            Operator::NotLike => String::from("not like"),
        }
    }
}

/// 基础条件，记录对应的字段、运算符以及数据值
#[derive(Debug)]
pub struct Cond<T: ToString> {
    key: String,
    op: Operator,
    value: T,
}

impl<T: ToString> Cond<T> {
    pub fn new(key: &str, op: Operator, value: T) -> Self {
        Self {
            key: key.to_string(),
            op,
            value,
        }
    }

    /// 解析最终的 sql 条件
    /// # Examples
    /// ```
    /// use lib_sql::utils::Operator;
    /// use lib_sql::utils::Cond;
    /// // id=1
    /// let cond = Cond::new("id", Operator::Eq, 1);
    /// ```
    fn parse(&self) -> String {
        match self.op {
            Operator::Ne
            | Operator::Eq
            | Operator::Ge
            | Operator::Gt
            | Operator::Le
            | Operator::Lt => format!(
                "({} {} {})",
                self.key,
                self.op.to_string(),
                self.value.to_string()
            ),
            Operator::In | Operator::NotIn => format!(
                "({} {} ({}))",
                self.key,
                self.op.to_string(),
                self.value.to_string()
            ),
            Operator::Land | Operator::LandNe => format!(
                "({} {} {} {})",
                self.key,
                self.op.to_string(),
                self.value.to_string(),
                self.value.to_string()
            ),
            Operator::Like | Operator::NotLike => format!(
                "({} {} \"%{}%\")",
                self.key,
                self.op.to_string(),
                self.value.to_string()
            ),
        }
    }
}

/// 条件解析器，支持复杂条件的拼接，通过 and/or 方法来组合条件，通过 parse 来解析最终用于 sql 查询的 where 条件
#[derive(Debug, Default, Clone)]
pub struct Matcher {
    conds_and: Vec<String>,
    conds_or: Vec<String>,
}

impl Matcher {
    /// 实例化
    pub fn new() -> Self {
        Matcher::default()
    }

    /// 判断条件是否为空
    pub fn is_empty(&self) -> bool {
        self.conds_and.is_empty() && self.conds_or.is_empty()
    }

    /// 清空所有已设置的条件
    pub fn clear(&mut self) {
        self.conds_and.clear();
        self.conds_or.clear();
    }

    /// 拼接 and 条件
    /// # Examples
    /// ```
    ///
    /// use lib_sql::utils::Operator;
    /// use lib_sql::utils::Matcher;
    /// let mut matcher = Matcher::new();
    /// matcher.and("id", Operator::Eq, 1);
    /// ```
    pub fn and<T: ToString>(&mut self, key: &str, op: Operator, value: T) -> &Self {
        let cond = Cond::new(key, op, value);
        self.conds_and.push(cond.parse());
        self
    }

    /// 拼接 and 条件
    /// # Examples
    /// ```
    ///
    /// use lib_sql::utils::Operator;
    /// use lib_sql::utils::Matcher;
    /// let mut matcher = Matcher::new();
    /// matcher.and_str("id", Operator::Eq, "value");
    /// ```
    pub fn and_str<T: ToString>(&mut self, key: &str, op: Operator, value: T) -> &Self {
        let cond = Cond::new(key, op, format!("'{}'", value.to_string()));
        self.conds_and.push(cond.parse());
        self
    }

    /// 拼接另一个 and 条件
    /// # Examples
    /// ```
    /// use lib_sql::utils::Operator;
    /// use lib_sql::utils::Search;
    /// use lib_sql::utils::Matcher;
    ///
    /// let mut matcher = Matcher::new();
    /// matcher.and("id", Operator::Eq, 1);
    /// matcher.and("name", Operator::Eq, "cat");
    ///
    /// let mut search = Search::default();
    /// search.matcher.and("id", Operator::In, "1,2,3,4");
    /// search.matcher.and_matcher(matcher);
    /// ```
    /// 最终的 where 条件如下：
    /// where id in (1,2,3,4) and (id=1 and name='cat')
    pub fn and_matcher(&mut self, matcher: Self) -> &Self {
        self.conds_and.push(matcher.parse());
        self
    }

    /// 拼接 or 条件
    /// # Examples
    /// ```
    /// use lib_sql::utils::Operator;
    /// use lib_sql::utils::Matcher;
    /// let mut matcher = Matcher::new();
    /// matcher.or("id", Operator::Eq, 1);
    /// ```
    pub fn or<T: ToString>(&mut self, key: &str, op: Operator, value: T) -> &Self {
        let cond = Cond::new(key, op, value);
        self.conds_or.push(cond.parse());
        self
    }

    /// 拼接另一个 or 条件
    /// # Examples
    /// ```
    /// use lib_sql::utils::Operator;
    /// use lib_sql::utils::Search;
    /// use lib_sql::utils::Matcher;
    /// let mut matcher = Matcher::new();
    /// matcher.and("id", Operator::Eq, 1);
    /// matcher.and("name", Operator::Eq, "cat");
    ///
    /// let mut search = Search::default();
    /// search.matcher.and("id", Operator::In, "1,2,3,4");
    /// search.matcher.or_matcher(matcher);
    /// ```
    /// 最终的 where 条件如下：
    /// where id in (1,2,3,4) or (id=1 and name='cat')
    pub fn or_matcher(&mut self, matcher: Self) -> &Self {
        self.conds_or.push(matcher.parse());
        self
    }

    /// 将条件最终解析为 sql 可用的条件
    pub(crate) fn parse(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        if !self.conds_and.is_empty() && !self.conds_or.is_empty() {
            format!(
                "({}) and ({})",
                self.conds_and.join(" and "),
                self.conds_or.join(" or ")
            )
        } else {
            if !self.conds_and.is_empty() {
                format!("({})", self.conds_and.join(" and "),)
            } else {
                format!("({})", self.conds_or.join(" or "))
            }
        }
    }
}

/// 排序器，用于组合最终提供给 sql 使用的排序语句
#[derive(Debug, Default, Clone)]
pub struct Comparator {
    list: Vec<String>,
}

impl Comparator {
    pub fn new() -> Self {
        Default::default()
    }

    /// 判断排序是否为空
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// 添加排序
    /// # Examples
    /// ```
    /// use lib_sql::utils::Comparator;
    /// // sql: order by id desc,name asc;
    /// let mut comp = Comparator::new();
    /// comp.add("id", true);
    /// comp.add("name", false);
    /// ```
    pub fn add(&mut self, key: &str, desc: bool) {
        self.list
            .push(format!("{} {}", key, if desc { "desc" } else { "asc" }));
    }

    /// 添加排序
    /// # Examples
    /// ```
    /// use lib_sql::utils::Comparator;
    /// // sql: order by id desc,name asc;
    /// let mut comp = Comparator::new();
    /// comp.insert(0, "id", true);
    /// comp.insert(1, "name", false);
    /// ```
    pub fn insert(&mut self, index: usize, key: &str, desc: bool) {
        self.list
            .insert(index, format!("{} {}", key, if desc { "desc" } else { "asc" }));
    }

    /// 清空排序字段
    pub fn clear(&mut self) {
        self.list.clear();
    }

    /// 解析出最终 sql 可用的排序语句，如：order by id desc,name asc;
    pub(crate) fn parse(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        format!(" order by {}", self.list.join(","))
    }
}

/// 查询条件
///
/// # Examples
/// 一个完整的查询条件如下
/// ```
///  use lib_sql::utils::Operator;
/// use lib_sql::utils::Search;
/// use lib_sql::utils::Matcher;
///  let mut search = Search::default();
///  search.matcher.and("id", Operator::Eq, 1); // 可选，条件，不提供查询条件则默认会查全表
///  search.sort.add("id", true); // 可选，排序
///  search.group = "name".to_string(); // 可选，group by
///  search.start = 0;  // 可选，分页
///  search.limit = 10; // 可选，一页拿 10 条数据
///  // 调用查询接口
///  // let list = dao.list(&mut search).unwrap();
///  // 获取查询的总数，只有指定了分页条件 start & limit 时，total 才会有值返回
///  let total = search.total;
/// ```
///
#[derive(Debug, Clone)]
pub struct Search {
    pub matcher: Matcher,
    pub fileds: String,
    pub group: String,
    pub start: isize,
    pub limit: isize,
    pub sort: Comparator,
    pub total: i64,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            matcher: Default::default(),
            fileds: "*".to_string(),
            group: Default::default(),
            start: -1,
            limit: -1,
            sort: Default::default(),
            total: Default::default(),
        }
    }
}

impl Search {
    pub fn new() -> Self {
        Default::default()
    }
    
    /// 判断是否提供了分页条件
    pub fn is_seach_total(&self) -> bool {
        self.start > -1 && self.limit > -1
    }

    /// 解析出最终 sql 可用的查询语句。如：
    /// select * from animals where id>0 group by name order by id desc,name asc limit 0,10;
    pub fn parse(&self, table: &str) -> String {
        let mut sql = format!("select {} from {}", self.fileds, table);

        // 条件
        if !self.matcher.is_empty() {
            let cond = self.matcher.parse();
            if !cond.is_empty() {
                sql.push_str(&format!(" where {}", cond));
            }
        }

        if !self.group.is_empty() {
            sql.push_str(" group by ");
            sql.push_str(&self.group);
        }
        if !self.sort.is_empty() {
            sql.push_str(&self.sort.parse());
        }
        if self.start > -1 && self.limit > -1 {
            sql.push_str(&format! {" limit {},{}", self.start, self.limit});
        }
        sql
    }
}
