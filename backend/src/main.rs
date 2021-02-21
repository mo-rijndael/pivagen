use std::net::{TcpListener, Shutdown};
use std::io::{self, Read, Write};

mod pivagen;

fn main() -> io::Result<()> {
    let mut generator = pivagen::Piva::new()?;
    let listener = TcpListener::bind("localhost:7482")?; // PIVA on T9 keyboard
    
    for connection in listener.incoming() {
        if let Ok(mut connection) = connection {
            let mut write_flag = [0u8; 1];
            let mut string = String::new();
            if let Err(e) = connection.read_exact(&mut write_flag) {
                eprintln!("Failed to read flag: {}", e);
                continue
            }
            if let Err(e) = connection.read_to_string(&mut string) {
                eprintln!("Failed to read flag: {}", e);
                continue
            }
            let [write_flag] =  write_flag;
            println!("{} {}",write_flag, string);
            if write_flag != 0 {
                if let Err(e) = generator.save_message(string) {
                    eprintln!("Saving failed: {}", e);
                }
                else {println!("Saved")}
            }
            else {
                let answer = generator.generate_answer(&string);
                if let Err(e) = connection.write_all(answer.as_bytes()) {
                    eprintln!("Sending answer to client failed: {}", e);
                }
                else {
                    connection.flush()?;
                    println!("Generated and sent")}
            }
            connection.shutdown(Shutdown::Both)?;
            drop(connection);
        }
    }
    Ok(())
}
