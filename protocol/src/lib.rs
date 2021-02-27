use std::io::{self, Read, Write};
use std::net::TcpStream;

mod traits;
pub use traits::{FromReader, ToWriter};

pub struct Request {
    pub write_intent: bool,
    pub content: String,
}
impl FromReader for Request {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let write_intent = bool::from_reader(reader)?;
        let content = String::from_reader(reader)?;
        Ok(Self {
            write_intent,
            content
        })
    }    
}
impl ToWriter for Request {
    fn to_writer<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.write_intent.to_writer(writer)?;
        self.content.to_writer(writer)?;
        Ok(())
    }
}
pub mod client {
    use super::*;
    pub fn save(content: &str) -> io::Result<()> {
        let mut writer = TcpStream::connect("localhost:7482")?;
        let content = content.to_owned();
        let request = Request {
            write_intent: true,
            content
        };
        request.to_writer(&mut writer)
    }
    pub fn generate(content: &str) -> io::Result<String> {
        let mut stream = TcpStream::connect("localhost:7482")?;
        let content = content.to_owned();
        let request = Request {
            write_intent: false,
            content,
        };
        request.to_writer(&mut stream)?;
        String::from_reader(&mut stream)
    }
}
