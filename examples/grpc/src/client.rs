use tonic::{Request, Status};
use tonic::codegen::InterceptedService;
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::transport::{Channel, Endpoint};
use uuid::Uuid;

use grpc::chat::chat_client::ChatClient;
use grpc::chat::ChatRequest;

#[derive(Default)]
struct TraceInterceptor {}

/// 使用 Interceptor trait 定义拦截器
impl Interceptor for TraceInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        let trace_id = Uuid::new_v4().to_string();

        let value = MetadataValue::try_from(trace_id).unwrap();

        req.metadata_mut().insert("trace-id", value);

        Ok(req)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoints = ["http://127.0.0.1:16444", "http://127.0.0.1:16443", ]
        .into_iter()
        .map(|addr| Endpoint::from_static(addr));

    // 客户端负载均衡
    let channel = Channel::balance_list(endpoints);

    let mut client: ChatClient<InterceptedService<Channel, TraceInterceptor>> =
        // ChatClient 是 tonic 生成的样板代码
        // 配置拦截器
        ChatClient::with_interceptor(channel, TraceInterceptor::default());

    for _ in 0..10 {
        let request = Request::new(ChatRequest {
            message: "Hello?".into(),
        });

        let response = client.unary_chat(request).await?;

        println!("{:#?}", response);
    }

    Ok(())
}


