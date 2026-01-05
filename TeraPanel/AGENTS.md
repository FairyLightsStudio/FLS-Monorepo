这是一个分布式服务器管理面板 TeraPanel

面板有的功能：

- 节点轻松运维（NodeAdmin）
  - FileManager
  - Terminal
  - SimpleObservability
    - 说人话就是让用户看到节点 CPU 占用、内存 占用、网络 I/O、磁盘 I/O 等信息
  - DepsInstaller
- 实例管理（InstanceController）
  - 以及一个实例附带的内网穿透、实例内文件管理（FileManager）的功能
  - 一个实例有多种运行时可选：Systemd、Container、Process、Nginx
    - Process：
      类似于 Supervisor 的 “裸进程” 管理器
      为无法使用 systemd/容器 的环境（windows、Android Termux、macOS ）兜底
    - Nginx：
      静态网站、php 网站 托管支持
- 内网穿透（IntranetPenetration）
  - 有一个有公网 ip 的节点、没有公网的节点可以通过内网穿透服务"用"这个公网 ip 暴露自己的端口

面板的技术架构：

- 消息中间件 —— NATS —— 负责 terapanel 和 terad 组群之间的通信
- 被分成了 terapanel、terad 两部分
  - terapanel
    - 实现 web 面板应用，通过 grpc-web、websocket 实现 Angular 前端和后端的通信，向用户呈现/允许用户改变节点及服务状态
    - 鉴权/授权：SimpleIdP 简单的单 admin 用户
      - 通过密码实现 admin 登录 、可选通过 OpenID / 一次性密码
    - 远期目标
      - ServiceProvider：可选 把 美西螈面板 的 Service 作为 服务提供者 上架到 🌌 服务大厅
      - UserCenter：可选 通过我的 UserCenter 项目替换 SimpleIdP，实现 多用户、ReBAC 授权、鉴权（若用户不启用 UserCenter，就回退到简单的单 admin 用户（SimpleIdP））
  - terad
    - 一个节点上部署一个 terad，负责管理本节点上的服务、实现本节点的 NodeAdmin、IntranetPenetration 功能
    - LINUX、windows、macOS 多系统支持

ServiceProvider 指的是对接到 我们另外一个、面向普通用户的商业化服务大厅系统，
UserCenter 指的是这种外部的 Authn/Authz 系统，例如 Keycloak，如果用户嫌麻烦懒得接的话就“回退到简单的单 admin 用户”，通过密码登录，并可选两步验证，在不接入第三方认证系统的情况下，其密码、两步验证数据存储在 terapanel 的 SqlLite 里面
