;; 仅导入真正需要的模块
(use-modules (guix profiles)     ; 提供 specifications->manifest
             (gnu packages))      ; 确保包可被发现

(specifications->manifest
 (list
  ;; --- 核心 Rust 工具链 ---
  "rust"
  "rust:cargo"
  "rust:tools"

  ;; --- 编译与链接必需 ---
  "gcc-toolchain"        ; 提供 cc、ld 等
  "pkg-config"           ; 帮助 Rust 的 -sys 包找到 C 库
  "openssl"              ; 很多网络 crate 依赖 OpenSSL

  ;; --- 服务与辅助工具 ---
  "nats-server"          ; NATS 消息中间件
  "just"                 ; 现代化的任务运行器
  "git"                  ; 版本控制

  ;; --- 可选：常用 Rust CLI 工具（取消注释即可安装）---
  ;; "bat"
  ;; "ripgrep"
  ;; "fd"
  ))
