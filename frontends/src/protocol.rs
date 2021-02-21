use std::net::{TcpStream, Shutdown};
use std::io::{self, Read, Write};

pub fn save(text: &str) -> io::Result<()> {
    println!("Enter save");
    let mut connection = TcpStream::connect("localhost:7482")?;
    let write_flag = 1u8;
    connection.write_all(&[write_flag])?;
    connection.write_all(text.as_bytes())?;
    connection.shutdown(Shutdown::Both)?;
    println!("Exit save");
    Ok(())
}
pub fn generate(text: &str) -> io::Result<String> {
    println!("Enter generate");
    let mut connection = TcpStream::connect("localhost:7482")?;
    let write_flag = 0u8;
    connection.write_all(&[write_flag])?;
    connection.write_all(text.as_bytes())?;
    connection.flush()?;
    println!("Bytes sent");
    let mut answer = String::new();
    connection.read_to_string(&mut answer)?;
    println!("Byted read");
    connection.shutdown(Shutdown::Both)?;
    println!("Conn closed");
    Ok(answer)
}
