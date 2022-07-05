use std::io;
use tokio::net::{TcpListener, TcpStream};

use protocol::asyncio::{Request, TcpReceive, TcpSend};

const DEFAULT_ANSWER: &str = "хуй тебе";

mod pivagen;

async fn handle_connection(
    mut connection: TcpStream,
    generator: &mut pivagen::Piva,
) -> io::Result<()> {
    let request = Request::receive(&mut connection).await?;
    if request.write_intent {
        generator.save_message(request.content)?;
    } else {
        let answer = generator
            .generate_answer(&request.content)
            .unwrap_or_else(|| DEFAULT_ANSWER.to_owned());
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
