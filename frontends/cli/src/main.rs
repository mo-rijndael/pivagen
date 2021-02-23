use protocol::*;
use std::io;
use std::net::TcpStream;

fn main() -> io::Result<()> {
    loop {
        let mut stream = TcpStream::connect("localhost:7482")?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let request = Request {
            write_intent: false,
            content: line
        };
        request.to_writer(&mut stream)?;
        let response = String::from_reader(&mut stream)?;
        println!("{}", response);

    }
}
