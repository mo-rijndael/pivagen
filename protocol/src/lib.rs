use std::io;
use tokio::net::TcpStream;
use async_trait::async_trait;

mod traits;
pub use traits::{TcpSend, TcpReceive};

pub struct Request {
    pub write_intent: bool,
    pub content: String,
}
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
        let request = Request {write_intent, content};
        Ok(request)
    }
}
pub mod client {
    use super::*;
    pub async fn save(content: &str) -> io::Result<()> {
        let mut writer = TcpStream::connect("localhost:7482").await?;
        let content = content.to_owned();
        let request = Request {
            write_intent: true,
            content
        };
        request.send(&mut writer).await
    }
    pub async fn generate(content: &str) -> io::Result<String> {
        let mut stream = TcpStream::connect("localhost:7482").await?;
        let content = content.to_owned();
        let request = Request {
            write_intent: false,
            content,
        };
        request.send(&mut stream).await?;
        String::receive(&mut stream).await
    }
}
