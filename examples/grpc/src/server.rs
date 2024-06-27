use tonic::{Request, Response, Status};
use tonic::transport::Server;

use grpc::chat::{ChatRequest, ChatResponse};
use grpc::chat::chat_server::{Chat, ChatServer};

/// 定义一个 ChatService
#[derive(Debug, Default)]
struct ChatService {}

/// 为 ChatService 实现 RPC 接口
#[tonic::async_trait]
impl Chat for ChatService {
    async fn unary_chat(
        &self,
        request: Request<ChatRequest>,
    ) -> Result<Response<ChatResponse>, Status> {
        let resp = ChatResponse {
            message: format!("Received: {}", request.into_inner().message)
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addrs = ["127.0.0.1:16443", "127.0.0.1:16444"];

    let handles = addrs.into_iter()
        .map(|addr| {
            let addr = addr.parse().unwrap();

            // ChatServer 是 tonic 生成的样板代码
            // 配置拦截器
            let svc = ChatServer::with_interceptor(
                ChatService::default(),
                intercept,
            );

            let server = Server::builder()
                .add_service(svc)
                .serve(addr);

            // 分别启动两个 server
            tokio::spawn(async move {
                if let Err(e) = server.await {
                    eprintln!("Error = {:?}", e);
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.await?;
    }

    Ok(())
}

/// 使用 FnMut trait 定义拦截器
fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    // 得到 metadata 中的 trace-id
    if let Some(v) = req.metadata().get("trace-id") {
        if let Ok(s) = v.to_str() {
            println!("{s} - {:?}", req);
        }
    }

    Ok(req)
}
