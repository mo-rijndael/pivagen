use std::io::{self, Error, ErrorKind};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};
use async_trait::async_trait;

#[async_trait]
pub trait TcpSend {
    async fn send(&self, stream: &mut TcpStream) -> io::Result<()>;
}
#[async_trait]
pub trait TcpReceive {
    async fn receive(stream: &mut TcpStream) -> io::Result<Self>
        where Self: Sized;
}

#[async_trait]
impl TcpSend for bool {
    async fn send(&self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_u8(*self as u8).await?;
        Ok(())
    }
}

#[async_trait]  
impl TcpSend for String {
    async fn send(&self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_u64(self.len() as u64).await?;
        stream.write_all(self.as_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl TcpReceive for bool {
    async fn receive(stream: &mut TcpStream) -> io::Result<Self> {
        let byte = stream.read_u8().await?;
        Ok(byte != 0)
    }
}

#[async_trait]
impl TcpReceive for String {
    async fn receive(stream: &mut TcpStream) -> io::Result<Self> {
        let len = stream.read_u64().await?;
        let mut buffer = vec![0u8; len as usize];
        stream.read_exact(&mut buffer).await?;
        let response = String::from_utf8(buffer).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(response)
    }
}
