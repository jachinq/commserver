use std::{fmt::Debug, fs, path::Path};

use sqlite::Connection;

use crate::{
    traits::{Connect, Error},
    utils::{Config, read_config},
};

#[derive(Debug)]
pub enum Mode {
    Mysql,
    Sqlite,
    PostgrepSql,
}

// T: 实现了 Connect 的连接实例，通过 T 来控制不同类型的数据库连接
pub struct Dao<T: Connect> {
    pub config: Config,
    pub connect: T,
}

// sqlite 的 dao
impl Connect for Connection {}
impl Dao<Connection> {
    /// 创建dao，使用默认配置文件进行连接，配置文件不能为空，否则报错。默认配置文件路径：./config.toml
    pub fn new() -> Result<Self, Error> {
        Dao::new_with_path("./conf/config.toml")
    }

    /// 创建dao，使用指定的配置文件进行连接，配置文件不能为空，否则报错。配置文件为 toml 格式
    pub fn new_with_path(config_path: &str) -> Result<Self, Error> {
        // 读取配置文件，连接数据库
        let config = read_config(config_path);
        if let Err(e) = config {
            return Err(e);
        }
        let config = config.unwrap();

        let datasource = &config.datasource;

        // 创建数据库的文件夹
        let dir_path = Path::new(datasource).parent().unwrap();
        if!dir_path.exists() {
            fs::create_dir_all(dir_path).unwrap();
        }

        let connect = sqlite::open(datasource).unwrap();
        Ok(Dao { config, connect })
    }
}
