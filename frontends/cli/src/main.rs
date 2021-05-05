use protocol::sync::*;
use std::io;
use std::net::TcpStream;

fn main() -> io::Result<()> {
    loop {
        let mut stream = TcpStream::connect("localhost:7482")?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        Request::generate(line).send(&mut stream)?;
        let response = String::receive(&mut stream)?;
        println!("{}", response);
    }
}
