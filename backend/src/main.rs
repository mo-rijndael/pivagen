use std::io::{self};
use std::net::{TcpListener, TcpStream};

use protocol::*;

mod pivagen;

fn handle_connection(mut connection: TcpStream, generator: &mut pivagen::Piva) -> io::Result<()> {
    let request = Request::from_reader(&mut connection)?;
    if request.write_intent {
        generator.save_message(request.content)?;
    } else {
        let answer = generator.generate_answer(&request.content);
        answer.to_writer(&mut connection)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut generator = pivagen::Piva::new()?;
    let listener = TcpListener::bind("localhost:7482")?; // PIVA on T9 keyboard

    for connection in listener.incoming() {
        if let Ok(connection) = connection {
            match handle_connection(connection, &mut generator) {
                Ok(_) => {}
                Err(e) => {
                    println!("Connection lost: {}", e)
                }
            }
        }
    }
    Ok(())
}
