use protocol::asyncio::client;
use rand::Rng;
use std::convert::From;
use std::fmt;
use std::io;
use teloxide::prelude::*;

#[derive(Debug)]
enum BotError {
    TelegramError(teloxide::RequestError),
    BackendError(io::Error),
}
impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TelegramError(e) => write!(f, "Problem with API: {}", e),
            Self::BackendError(e) => write!(f, "Problem with backend: {}", e),
        }
    }
}
impl From<teloxide::RequestError> for BotError {
    fn from(e: teloxide::RequestError) -> Self {
        Self::TelegramError(e)
    }
}
impl From<io::Error> for BotError {
    fn from(e: io::Error) -> Self {
        Self::BackendError(e)
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let token = std::env::var("TOKEN").expect("No TOKEN specified");
    let bot = Bot::new(token);
    let my_id = bot
        .get_me()
        .send()
        .await
        .expect("Failed to get self")
        .user
        .id;
    teloxide::repl(bot, move |event| async move {
        if let Err(e) = handle_message(event, my_id).await {
            eprintln!("{:#}", e);
        };
        Result::<_, BotError>::Ok(())
    })
    .await;
}

type Cx = UpdateWithCx<Bot, Message>;
fn should_reply(m: &Cx, my_id: i64) -> bool {
    let is_private = m.update.chat.is_private();
    let is_command = m.update.text().map_or(false, |s| s.starts_with("/pivagen"));
    let is_reply_to_mine = m.update.from().map_or(false, |user| user.id == my_id);
    let random = rand::thread_rng().gen_bool(0.15);
    is_private || is_reply_to_mine || is_command || random
}

async fn handle_message(event: Cx, my_id: i64) -> Result<(), BotError> {
    if let Some(text) = event.update.text() {
        println!("Should save: {}", text);
        client::save(text).await?;
    };
    if should_reply(&event, my_id) {
        let text = event.update.text().unwrap_or("");
        println!("Must generate");
        let reply = client::generate(text).await?;
        println!("Generated {}", reply);
        event.answer(reply).send().await?;
    };
    Ok(())
}
