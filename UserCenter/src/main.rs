use buffa::view::OwnedView;
use connectrpc::{RequestContext, Response, Router, ServiceResult};
use std::sync::Arc;

pub mod proto {
    connectrpc::include_generated!();
}
use proto::greet::v1::*;

struct MyGreet;

impl GreetService for MyGreet {
    async fn greet(
        &self,
        _ctx: RequestContext,
        req: OwnedView<GreetRequestView<'static>>,
    ) -> ServiceResult<GreetResponse> {
        // req 解引用为 view：零拷贝字段访问。
        // 字符串字段是自请求缓冲区借用的 &str。
        Response::ok(GreetResponse {
            greeting: format!("Hello, {}!", req.name),
            ..Default::default()
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = Arc::new(MyGreet);
    let router = service.register(Router::new());
    let app = router.into_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8082").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
