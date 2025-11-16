use crate::model::User;
use lib_date::Calendar;
use std::future::{Ready, ready};

use actix_web::{Error, FromRequest, HttpRequest, Result, error};

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, errors::Error as JwtError,
};
use serde::{Deserialize, Serialize};


// 简单的密码加密函数（实际项目中应使用更安全的加密方法）
pub fn hash_password(password: &str) -> String {
    // 这里使用简单的加密，实际应使用 bcrypt 等安全的哈希函数
    format!("{:x}", md5::compute(password.as_bytes()))
}

// 验证密码
pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}

// 生成 Token（简单实现，实际项目中应使用 JWT）
pub fn generate_token(user_id: i32) -> String {
    format!("{}-{}", user_id, Calendar::now().timestamp())
}

// 初始化用户
pub fn init_user() -> User {
    User {
        id: 1,
        username: "demo".to_string(),
        password: hash_password("demo123"),
        email: "".to_string(),
        created_at: Calendar::now().timestamp() as i32,
    }
}




const JWT_SECRET: &[u8] = b"C8pu5q5PQKAY1Uzpi2c7VqInOkadTh6l";

/// 创建token
pub fn create_jwt(id: &i32, remember: bool) -> String {
    let expire_sec = if remember {
        3600 * 24 * 365 // 1y
    } else {
        3600 // 1h
    };
    let expiration = (Calendar::now().timestamp() + expire_sec * 1000) as usize;

    let header = Header::new(Algorithm::HS512);
    let claims = Claims::new(id, expiration);

    let token = jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        // .map(|s| format!("Bearer {}", s))
        .unwrap();
    // println!("---{}---", token);
    token
}

/// 验证token
pub fn validate_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    let validation = Validation::new(Algorithm::HS512);
    let key = DecodingKey::from_secret(JWT_SECRET);
    let data = jsonwebtoken::decode::<Claims>(token, &key, &validation)?;
    Ok(data)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    iss: String,
    pub exp: usize,
    /// 保存的用户id
    pub id: i32,
}

impl Claims {
    pub fn new(id: &i32, exp: usize) -> Self {
        Self {
            iss: "test".to_owned(),
            id: *id,
            exp,
        }
    }
}

impl FromRequest for User {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        // println!("get UserData from request");
        ready({
            let auth = req.headers().get("Authorization");
            if let Some(val) = auth {
                let token = val
                    .to_str()
                    .unwrap()
                    .split("Bearer ")
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap();
                let result = validate_token(token);
                match result {
                    Ok(data) => {
                        let mut user = User::default();
                        user.id = data.claims.id;
                        Ok(user)
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        Err(error::ErrorBadRequest("Invalid Authorization"))
                    }
                }
            } else {
                Err(error::ErrorUnauthorized("Authorization Not Found"))
            }
        })
    }
}
