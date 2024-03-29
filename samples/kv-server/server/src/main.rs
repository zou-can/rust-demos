use crate::storage::memory::Memory;
use crate::storage::Storage;
use std::sync::Arc;
use tracing::{debug, error, info};
use uuid::Uuid;
use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use kv_core::domain::{Request, Response};

mod command_handler;
mod storage;

/// 实际的 Server 类
struct Server<Store> {
    storage: Store,
}


struct SharedServer<Store = Memory> {
    // 多线程共享
    shared: Arc<Server<Store>>,
}


impl<Store: Storage> SharedServer<Store> {
    pub fn new(storage: Store) -> Self {
        let server = Server { storage };

        Self {
            shared: Arc::new(server),
        }
    }

    fn handle(&self, request: Request) -> Response {
        let req_id = Uuid::new_v4();

        debug!("{req_id} - request = {:?}", request);

        // TODO: 发送 on_received 事件
        let response = command_handler::handle(request, &self.shared.storage);

        debug!("{req_id} - response = {:?}", response);

        // TODO: 发送 on_executed 事件

        response
    }
}

/// 实现 Clone trait
impl<Storage> Clone for SharedServer<Storage> {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let server = SharedServer::new(Memory::new());

    let addr = "127.0.0.1:12345";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {addr}");

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);

        let svr = server.clone();
        tokio::spawn(async move {
            // 封装数据流，用于处理 ProtoBuf 报文
            // todo 接收并处理数据
            info!("Client {:?} disconnected", addr);
        });
    }
}

