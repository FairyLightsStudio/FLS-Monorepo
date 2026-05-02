mod config;
mod state;
mod api;

use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, error};
use volo_http::Router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ====================================================
    // 1. 初始化日志 (Observability)
    // ====================================================
    // 允许通过环境变量 RUST_LOG=debug 来控制日志级别
    tracing_subscriber::fmt::init();
    info!("🚀 TeraPanel 正在启动...");

    // ====================================================
    // 2. 加载配置 (Config)
    // ====================================================
    let settings = config::Settings::new().expect("无法加载配置文件");
    info!("配置加载成功: 监听于 {}:{}", settings.server.host, settings.server.port);

    // ====================================================
    // 3. 基础设施连接 (Infrastructure)
    // ====================================================
    // 3.1 连接 PostgreSQL
    info!("正在连接数据库...");
    let db_pool = sqlx::PgPool::connect(&settings.database.url).await
        .expect("无法连接到数据库");
    
    // 3.2 连接 NATS
    info!("正在连接 NATS...");
    let nats_client = async_nats::connect(&settings.nats.url).await
        .expect("无法连接到 NATS 服务器");

    // ====================================================
    // 4. 构建应用状态 (State)
    // ====================================================
    let state = state::AppState {
        db: db_pool,
        nats: nats_client,
    };

    // ====================================================
    // 5. 构建路由与启动服务器 (Server)
    // ====================================================
    // 定义路由，并将 state 注入到层中 (Layer) 
    // 注意：Volo-HTTP 的状态注入方式可能随版本不同，
    // 通常可以通过闭包 move state 或者使用 Extension 扩展
    let app = Router::new()
        .route("/", volo_http::route::get(handler_home));
        // .layer(...) // 如果需要注入 state，可以使用中间件机制

    let addr: SocketAddr = format!("{}:{}", settings.server.host, settings.server.port)
        .parse()?;

    info!("✅ TeraPanel 启动成功! http://{}", addr);

    // 启动 Volo-HTTP 服务
    volo_http::Server::new(app)
        .run(addr)
        .await?;

    Ok(())
}

// 简单的 Handler 示例
async fn handler_home() -> &'static str {
    "Welcome to TeraPanel API!"
}