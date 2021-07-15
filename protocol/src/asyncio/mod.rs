use async_trait::async_trait;
use std::io;
use tokio::net::TcpStream;

pub use crate::request::Request;

mod traits;
pub use traits::{TcpReceive, TcpSend};

#[async_trait]
impl TcpSend for Request {
    async fn send(&self, stream: &mut TcpStream) -> io::Result<()> {
        self.write_intent.send(stream).await?;
        self.content.send(stream).await?;
        Ok(())
    }
}

#[async_trait]
impl TcpReceive for Request {
    async fn receive(stream: &mut TcpStream) -> io::Result<Self> {
        let write_intent = bool::receive(stream).await?;
        let content = String::receive(stream).await?;
        let request = Request {
            write_intent,
            content,
        };
        Ok(request)
    }
}
pub mod client {
    use super::*;
    pub async fn save(content: &str) -> io::Result<()> {
        let mut writer = TcpStream::connect("localhost:7482").await?;
        let content = content.to_owned();
        Request::save(content).send(&mut writer).await
    }
    pub async fn generate(content: &str) -> io::Result<String> {
        let mut stream = TcpStream::connect("localhost:7482").await?;
        let content = content.to_owned();
        Request::generate(content).send(&mut stream).await?;
        let response = String::receive(&mut stream).await?;
        Ok(response)
    }
}
