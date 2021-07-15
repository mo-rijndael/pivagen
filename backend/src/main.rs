use std::io::{self};
use tokio::net::{TcpListener, TcpStream};

use protocol::asyncio::{Request, TcpReceive, TcpSend};

mod pivagen;

async fn handle_connection(
    mut connection: TcpStream,
    generator: &mut pivagen::Piva,
) -> io::Result<()> {
    let request = Request::receive(&mut connection).await?;
    dbg!(&request);
    if request.write_intent {
        generator.save_message(request.content)?;
    } else {
        let answer = generator.generate_answer(&request.content);
        answer.send(&mut connection).await?;
    }
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let mut generator = pivagen::Piva::new()?;
    let listener = TcpListener::bind("localhost:7482").await?; // PIVA on T9 keyboard

    loop {
        let connection = listener.accept().await;
        if let Ok((connection, _)) = connection {
            match handle_connection(connection, &mut generator).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Connection lost: {}", e)
                }
            };
        }
    }
}
