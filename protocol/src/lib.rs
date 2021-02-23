use std::io::{self, Read, Write};

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
    pub fn save<W: Write>(content: &str, writer: &mut W) -> io::Result<()> {
        let content = content.to_owned();
        let request = Request {
            write_intent: true,
            content
        };
        request.to_writer(writer)
    }
    pub fn generate<S>(content: &str, stream: &mut S) -> io::Result<String>
        where S: Read + Write
    {
        let content = content.to_owned();
        let request = Request {
            write_intent: false,
            content,
        };
        request.to_writer(stream)?;
        String::from_reader(stream)
    }
}
