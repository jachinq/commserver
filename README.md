# CommServer

CommServer 是一个基于 Rust 语言的后端服务器，提供功能如下：

- 基于了基于 Oauth2.0 的用户认证和授权功能
- 提供了基于 Sqlite 的数据库存储功能

项目结构：

- app: 项目的主要代码，包括路由、控制器、服务等
- conf: 项目的配置文件，包括数据库配置、日志配置等
- crates: 项目的依赖库，包括数据库、日志、日期处理、json 解析等
  - lib-date: 一个日期处理库，支持常见的日期格式转换。
  - lib-json: 一个 json 解析库，支持将字符串解析为 json 对象。
  - lib-log: 一个日志库，支持将日志输出到控制台、文件等。
  - lib-sql：一个 sql 关系映射库，支持将默认常见的数据库增删查改，同时封装了返回对象。


## 构建

编译命令

```
cargo build --release --target x86_64-unknown-linux-musl
```

在 `target/x86_64-unknown-linux-musl/release` 下找到生成可执行文件。

将可执行文件 copy 到 docker 路径下，然后构建 docker 镜像

```
cd docker
docker build -t billreco-server .
```

将根路径下 `conf/config.toml` 复制到 `docker/conf` 下，运行镜像

```
docker compose up -d
```