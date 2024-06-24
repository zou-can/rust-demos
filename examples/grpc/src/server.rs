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
        println!("Received a request: {:#?}", request);

        let resp = ChatResponse {
            message: format!("Received: {}", request.into_inner().message)
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:16443".parse()?;

    // ChatServer 是 tonic 生成的样板代码
    let svc = ChatServer::new(ChatService::default());

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
