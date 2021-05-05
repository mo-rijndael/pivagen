use std::io::{self, Error, ErrorKind};
use std::io::{Read, Write};
use std::net::TcpStream;

pub trait TcpSend {
    fn send(&self, stream: &mut TcpStream) -> io::Result<()>;
}
pub trait TcpReceive {
    fn receive(stream: &mut TcpStream) -> io::Result<Self>
    where
        Self: Sized;
}

impl TcpSend for bool {
    fn send(&self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_all(&[*self as u8])?;
        Ok(())
    }
}

impl TcpSend for String {
    fn send(&self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_all(&(self.len() as u64).to_be_bytes())?;
        stream.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl TcpReceive for bool {
    fn receive(stream: &mut TcpStream) -> io::Result<Self> {
        let mut buffer = [0u8; 1];
        stream.read_exact(&mut buffer)?;
        let [byte] = buffer;
        Ok(byte != 0)
    }
}

impl TcpReceive for String {
    fn receive(stream: &mut TcpStream) -> io::Result<Self> {
        let mut buffer = [0u8; 8];
        stream.read_exact(&mut buffer)?;
        let len = u64::from_be_bytes(buffer);
        let mut buffer = vec![0u8; len as usize];
        stream.read_exact(&mut buffer)?;
        let response =
            String::from_utf8(buffer).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(response)
    }
}
