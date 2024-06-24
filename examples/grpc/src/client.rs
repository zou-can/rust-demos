use tonic::Request;
use grpc::chat::chat_client::ChatClient;
use grpc::chat::ChatRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        // ChatClient 是 tonic 生成的样板代码
        ChatClient::connect("http://127.0.0.1:16443").await?;

    let request = Request::new(ChatRequest {
        message: "Hello?".into(),
    });

    let response = client.unary_chat(request).await?;

    println!("{:#?}", response);

    Ok(())
}
