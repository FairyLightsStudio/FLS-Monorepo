FairyUserCenter 是一款开源且**开放**的 用户 身份核验（Authentication，简称Authn）、授权（Authorization，简称Authz）服务器，你可以让你的 web服务们 依赖它，这样你就不需要自己实现用户登录注册相关的逻辑啦。 

它是 BetterInternet 计划的子项目。

它有如下特色设计：

- 轻量化、高性能
- Authz 模块 围绕 GNAP 设计 
- 使用 状态机 设计 Authn 模块 

为满足常见用例，它还附带如下组件：

- 一个 "个人中心" 应用，允许用户管理自己的账号。
- 一些 Web Component ，以方便你将 FairyUserCenter 集成到你的 web服务。


```mermaid
erDiagram
    %% --- 客户端相关实体 (Client-Related Entities) ---
    client_instances {
        BIGINT id PK "内部唯一标识符"
        VARCHAR(255) instance_identifier UK "动态分配给客户端的实例 ID"
        VARCHAR(255) class_id "客户端软件标识符"
        VARCHAR(255) display_name "客户端显示名称"
        VARCHAR(2048) display_uri "客户端信息页面 URI"
        VARCHAR(2048) logo_uri "客户端 Logo URI"
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    client_keys {
        BIGINT id PK "内部唯一标识符"
        BIGINT client_instance_id FK "关联的客户端实例"
        VARCHAR(255) key_identifier "密钥标识符 (kid)"
        JSON jwk "JWK 格式的公钥"
        VARCHAR(50) proof_method "密钥持有证明方法"
        ENUM status "密钥状态 (active, revoked)"
        TIMESTAMP created_at
    }

    %% --- 授权流程核心实体 (Core Grant Flow Entities) ---
    grant_requests {
        BIGINT id PK "内部唯一标识符"
        BIGINT client_instance_id FK "发起请求的客户端实例"
        BIGINT user_id FK "请求所代表的资源所有者"
        ENUM status "授权请求状态 (processing, pending, approved, finalized)"
        VARCHAR(255) interaction_client_nonce
        VARCHAR(255) interaction_as_nonce
        VARCHAR(255) interaction_reference UK
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    requested_access_rights {
        BIGINT id PK "内部唯一标识符"
        BIGINT grant_request_id FK "关联的授权请求"
        VARCHAR(255) requested_label "客户端指定的标签"
        ENUM right_type "权限类型 (string, object)"
        VARCHAR(1024) reference_string "权限引用字符串"
        JSON details "权限对象的 JSON 结构"
    }

    access_tokens {
        BIGINT id PK "内部唯一标识符"
        BIGINT grant_request_id FK "颁发此令牌的授权请求"
        VARCHAR(512) token_value UK "令牌的值"
        ENUM token_type "令牌类型 (resource, continuation, management)"
        VARCHAR(255) label "客户端提供的标签"
        ENUM status "令牌状态 (active, revoked, expired)"
        JSON flags "与令牌关联的标志"
        BIGINT bound_client_key_id FK "令牌绑定的客户端密钥"
        BIGINT management_token_id FK "管理的令牌ID (自引用)"
        TIMESTAMP issued_at
        TIMESTAMP expires_at
    }

    granted_access_rights {
        BIGINT id PK "内部唯一标识符"
        BIGINT access_token_id FK "关联的访问令牌"
        ENUM right_type "权限类型 (string, object)"
        VARCHAR(1024) reference_string "权限引用字符串"
        JSON details "权限对象的 JSON 结构"
    }

    %% --- 用户/资源所有者相关实体 (User/Resource Owner Entities) ---
    users {
        BIGINT id PK "内部唯一标识符"
        VARCHAR(255) username UK
        JSON profile_info "用户资料信息"
        TIMESTAMP created_at
    }

    user_identifiers {
        BIGINT id PK "内部唯一标识符"
        BIGINT user_id FK "关联的用户"
        VARCHAR(100) format "主体标识符的格式"
        VARCHAR(1024) identifier_value "标识符的值"
    }

    %% --- 资源服务器相关实体 (Resource Server Entities) ---
    resource_servers {
        BIGINT id PK "内部唯一标识符"
        VARCHAR(255) instance_identifier UK "RS 实例标识符"
        VARCHAR(255) name "RS 名称"
        TIMESTAMP created_at
    }

    resource_server_keys {
        BIGINT id PK "内部唯一标识符"
        BIGINT resource_server_id FK "关联的资源服务器"
        VARCHAR(255) key_identifier "密钥标识符 (kid)"
        JSON jwk "JWK 格式的公钥"
        VARCHAR(50) proof_method "密钥持有证明方法"
        ENUM status "密钥状态 (active, revoked)"
        TIMESTAMP created_at
    }

    registered_resource_sets {
        BIGINT id PK "内部唯一标识符"
        BIGINT resource_server_id FK "注册此资源集的 RS"
        VARCHAR(255) resource_reference UK "代表资源集的引用字符串"
        TIMESTAMP created_at
    }

    registered_resource_rights {
        BIGINT id PK "内部唯一标识符"
        BIGINT resource_set_id FK "关联的已注册资源集"
        ENUM right_type "权限类型 (string, object)"
        VARCHAR(1024) reference_string "权限引用字符串"
        JSON details "权限对象的 JSON 结构"
    }

    %% --- 实体间关系 (Relationships) ---
    client_instances ||--o{ client_keys : "拥有 (has)"
    client_instances ||--o{ grant_requests : "发起 (initiates)"
    users ||--o{ grant_requests : "拥有 (owns)"
    users ||--o{ user_identifiers : "拥有 (has)"
    grant_requests ||--o{ requested_access_rights : "包含 (includes)"
    grant_requests ||--o{ access_tokens : "颁发 (issues)"
    access_tokens ||--o{ granted_access_rights : "包含 (includes)"
    client_keys ||--o{ access_tokens : "绑定到 (is bound to)"
    access_tokens }o--|| access_tokens : "管理 (manages)"
    resource_servers ||--o{ resource_server_keys : "拥有 (has)"
    resource_servers ||--o{ registered_resource_sets : "注册 (registers)"
    registered_resource_sets ||--o{ registered_resource_rights : "包含 (includes)"
```


```mermaid
sequenceDiagram
    participant User as 用户 (资源所有者)
    participant Client as 客户端实例
    participant AS as 授权服务器 (AS)
    participant RS as 资源服务器 (RS)

    %% Phase 1: 授权请求与用户交互 %%
    rect rgb(220, 230, 255)
        note over Client, AS: 阶段 1: 客户端发起授权请求
        Client->>AS: 发起授权请求 (POST /tx)<br/>请求访问RS上的资源 
        note right of Client: 包含: access_token (权限), client (身份), interact (交互方式) [cite: 573, 577, 579]

        AS->>Client: 响应: 需要用户交互 [cite: 2729]
        note left of AS: 包含: interact.redirect (交互URI), continue (继续请求所需信息) [cite: 832, 833]
    end

    rect rgb(230, 255, 230)
        note over User, AS: 阶段 2: 用户交互与授权
        Client->>User: 重定向用户到AS进行授权 [cite: 355]
        activate User
        User->>AS: 用户登录并同意授权 
        deactivate User
        AS->>User: AS将用户重定向回客户端 [cite: 359]
        note right of AS: 回调URI中包含 interact_ref 和 hash [cite: 360, 361]
        User->>Client: 浏览器访问客户端回调URI
    end

    %% Phase 2: 获取访问令牌 %%
    rect rgb(255, 240, 220)
        note over Client, AS: 阶段 3: 客户端获取访问令牌
        activate Client
        Client->>Client: 验证回调请求中的 hash 值 [cite: 363]
        Client->>AS: 继续授权请求 (POST /continue)<br/>提交 interact_ref [cite: 366, 1248, 2740]
        deactivate Client
        AS->>Client: 颁发访问令牌 (Access Token) 
        note left of AS: 令牌对客户端来说是不透明的 [cite: 863, 2925]
    end

    %% Phase 3: 访问受保护资源 %%
    rect rgb(255, 225, 225)
        note over Client, RS: 阶段 4: 客户端使用令牌访问资源
        Client->>RS: 请求受保护的资源<br/>(在Authorization头中携带Access Token) 
        note right of Client: 对于绑定密钥的令牌，请求必须被签名 
    end

    %% Phase 4: 资源服务器验证令牌 %%
    rect rgb(230, 245, 255)
        note over AS, RS: 阶段 5: RS通过内省验证令牌
        activate RS
        RS->>RS: 检查客户端请求的签名 (密钥证明) 
        RS->>AS: 请求令牌内省 (POST /introspect)<br/>发送客户端提供的Access Token [cite: 3091, 3092]
        note right of RS: RS使用自己的密钥对内省请求进行签名 [cite: 3075, 3092]

        AS->>RS: 返回内省结果 [cite: 3093]
        note left of AS: 包含: active (是否有效), access (权限), key (绑定的密钥)等信息 [cite: 3107, 3110, 3113]
    end

    %% Phase 5: 返回资源 %%
    rect rgb(245, 245, 220)
        note over Client, RS: 阶段 6: RS返回资源给客户端
        RS->>RS: 验证令牌权限是否满足当前请求
        RS->>Client: 返回受保护的资源 
        deactivate RS
    end
```