use lazy_static::lazy_static;
use protocol::asyncio::client;
use rand::Rng;
use std::env;
use std::error::Error;
use tokio_postgres::NoTls;

mod api;
use api::*;

mod metrics;

lazy_static! {
    static ref TOKEN: String = env::var("TOKEN").expect("No TOKEN set");
    static ref DB_URL: String = env::var("DB_URL").expect("Failed to get DB_URL");
}

fn should_reply(message: &Message, my_id: i32) -> bool {
    message.is_private()
        || message.text.contains(&format!("[club{}|", my_id))
        || (message.reply_message.is_some()
            && message.reply_message.as_ref().unwrap().from_id == -my_id)
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rand = rand::thread_rng();
    let group_id = get_me().await?.response[0].id;
    let client = reqwest::Client::new();
    let (database, connection) = tokio_postgres::connect(&DB_URL, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection lost {}", e);
        }
    });
    let mut longpoll = LongPoll::new(group_id).await?;
    let backend_unavailable = String::from("Ааеаоаооаа");

    loop {
        let events = match longpoll.get_events(&client).await {
            Ok(events) => events,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        for m in events {
            use metrics::{write_message, Chat, MsgType};
            if m.is_from_user() {
                let res = client::save(&m.text).await;
                if let Err(e) = res {
                    println!("Failed to save: {}", e);
                }
            }
            let chat_type = if m.is_private() {
                Chat::Private
            } else {
                Chat::Group
            };
            write_message(&database, MsgType::Incoming, chat_type).await?;
            if should_reply(&m, group_id) {
                let reply = match client::generate(&m.text).await {
                    Ok(reply) => reply,
                    Err(e) => {
                        eprintln!("{}", e);
                        backend_unavailable.clone()
                    }
                };
                m.reply(&reply, &client).await;
                write_message(&database, MsgType::Outgoing, chat_type).await?;
            } else if rand.gen_bool(0.05) {
                if let Ok(reply) = client::generate(&m.text).await {
                    m.reply(&reply, &client).await;
                    write_message(&database, MsgType::Outgoing, chat_type).await?;
                }
            }
        }
    }
}
