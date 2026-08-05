# 📜 Contracts - API 定义

![Protocol](https://img.shields.io/badge/type-Protocol-blue.svg)

> 本目录包含用户中心系统的协议定义文件

##
// session_token、user_id通过http only的cookie下发

## 🔐 身份认证协议 (authn)

`authn` 文件夹存放身份提供者使用的协议，实现以下功能：

- 用户注册
- 用户登录
- 用户中心身份管理

```rust
// 实现于 
usercenter/services/identity-provider
```

## 🔑 授权协议 (authz)

`authz` 文件夹存放授权服务器使用的协议，实现以下功能：

| 功能 | 描述 |
|------|------|
| 跨应用数据授权 | 授权应用A(客户端)读写当前用户在应用B(资源服务器)上的数据 |
| 个人信息授权 | 授权应用A读取当前用户在用户中心上的个人信息 |
| 敏感操作确认 | 为应用A确认敏感操作确实由当前用户发起 |

⚠️ **注意**：`authz`文件夹并不存放 OAuth2、OpenID compatible 的 API 定义。

```rust
// 实现于 
usercenter/services/authorization-server
```


## 用户设定 user

本文件夹允许更新用户信息

## 🛠️ 开发指南

1. 协议使用 Protocol Buffers 定义
2. 通过 volo 工具生成对应语言代码
3. 各服务实现对应协议功能
