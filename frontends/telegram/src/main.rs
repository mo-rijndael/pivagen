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
    teloxide::repl(bot, move |message: Message, bot: Bot| async move {
        if let Some(text) = message.text() {
            client::save(text).await?;
        };
        if should_reply(&message, my_id) {
            let text = message.text().unwrap_or("");
            let reply = client::generate(text).await?;
            bot.send_message(message.chat.id, reply).send().await?;
        };
        Result::<(), BotError>::Ok(())
    })
    .await;
}

fn should_reply(m: &Message, my_id: UserId) -> bool {
    let is_private = m.chat.is_private();
    let is_command = m.text().map_or(false, |s| s.starts_with("/pivagen"));
    let is_reply_to_mine = m.from().map_or(false, |user| user.id == my_id);
    let random = rand::thread_rng().gen_bool(0.15);
    is_private || is_reply_to_mine || is_command || random
}
