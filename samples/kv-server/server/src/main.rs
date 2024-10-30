use std::net::SocketAddr;
use crate::storage::memory::Memory;
use crate::storage::Storage;
use std::sync::Arc;
use tracing::{debug, error, info, trace};
use uuid::Uuid;
use anyhow::Result;
use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use kv_core::domain::{Request, Response};

mod request_handler;
mod storage;
mod serializer;

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

    async fn handle_connection(&self, mut socket: TcpStream, addr: SocketAddr) {
        let (mut reader, mut writer) = socket.split();
        let mut buf = BytesMut::with_capacity(1024);

        loop {
            match reader.read_buf(&mut buf).await {
                Ok(0) => {
                    trace!("Read data from {addr} finished.");
                    break;
                }
                Ok(n) => {
                    trace!("Read data from {addr}, data size = {n}.");

                    // TODO 序列化数据

                    match writer.write_all_buf(&mut buf).await {
                        Ok(_) => trace!("Write data to {addr} finished."),
                        Err(e) => error!("Write data to {addr} failed: {e:?}"),
                    }
                }
                Err(e) => {
                    error!("Read data from {addr} failed: {e:?}");
                    break;
                }
            }
        }


        trace!("Client {:?} disconnected.", addr);
    }

    fn handle_request(&self, request: Request) -> Response {
        // 序列化

        let req_id = Uuid::new_v4();

        debug!("{req_id} - request = {:?}", request);

        // TODO: 发送 on_received 事件
        let response = request_handler::handle(request, &self.shared.storage);

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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let server = SharedServer::new(Memory::new());

    let addr = "127.0.0.1:6736";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {addr}");

    loop {
        let (socket, addr) = listener.accept().await?;
        trace!("Client {:?} connected.", addr);

        let svr = server.clone();

        tokio::spawn(async move {
            svr.handle_connection(socket, addr).await;
        });
    }
}

