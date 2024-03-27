// 使用 include! 宏导入 protoc 编译得到的 rust 代码
// "OUT_DIR" 是默认的 protobuf rust 代码输出位置。
// 注意 env! 宏获取的是编译期的环境变量，而 std::env::var() 函数获取的是运行时的环境变量
include!(concat!(env!("OUT_DIR"), "/rpc.proto.rs"));

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use prost::Message;

    use crate::rpc;

    #[test]
    fn proto_message_test() {
        let req = rpc::SearchRequest {
            query: String::from("query string"),
            ..Default::default()
        };

        // 序列化
        let mut buffer = BytesMut::new();

        req.encode(&mut buffer).unwrap();

        println!("Serialized bytes for SearchRequest: {}", hex::encode(&buffer));

        // 反序列化

        let obj = rpc::SearchRequest::decode(&mut buffer).unwrap();
        println!("Deserialized result for SearchRequest: {:?}", obj);
    }
}