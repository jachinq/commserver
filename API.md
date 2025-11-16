# API

## 注册

```
POST /api/auth/register
```

请求参数：

- `username`：用户名
- `email`：用户名
- `password`：密码
- `confirm_password`：密码

响应参数：

- `user`: 用户信息
  - `id`: 用户ID
  - `username`: 用户名
  - `email`: 用户邮箱
  - `created_at`: 注册时间
- `token`：注册成功后返回的token

## 登录

```
POST /api/auth/login
```

请求参数：

- `username`：用户名
- `password`：密码
- `remember`: 是否记住登录状态

响应参数：

- `user`: 用户信息
  - `id`: 用户ID
  - `username`: 用户名
  - `email`: 用户邮箱
  - `created_at`: 注册时间
- `token`：注册成功后返回的token

