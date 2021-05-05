use std::io;
use std::net::TcpStream;

pub use super::request::Request;

mod traits;
pub use traits::{TcpReceive, TcpSend};

impl TcpSend for Request {
    fn send(&self, stream: &mut TcpStream) -> io::Result<()> {
        self.write_intent.send(stream)?;
        self.content.send(stream)?;
        Ok(())
    }
}

impl TcpReceive for Request {
    fn receive(stream: &mut TcpStream) -> io::Result<Self> {
        let write_intent = bool::receive(stream)?;
        let content = String::receive(stream)?;
        let request = Request {
            write_intent,
            content,
        };
        Ok(request)
    }
}
pub mod client {
    use super::*;
    pub fn save(content: &str) -> io::Result<()> {
        let mut writer = TcpStream::connect("localhost:7482")?;
        let content = content.to_owned();
        Request::save(content).send(&mut writer)
    }
    pub fn generate(content: &str) -> io::Result<String> {
        let mut stream = TcpStream::connect("localhost:7482")?;
        let content = content.to_owned();
        Request::generate(content).send(&mut stream)?;
        String::receive(&mut stream)
    }
}
