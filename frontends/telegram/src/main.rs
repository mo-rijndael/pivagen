use protocol::client;
use rand::Rng;
use teloxide::requests::{Requester, Request};
use std::io;

const TOKEN: &str = env!("TOKEN");

#[derive(Debug)]
enum BotError {
    TelegramError(teloxide::RequestError),
    BackendError(io::Error)
}
impl std::fmt::Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TelegramError(e) => write!(f, "Problem with API: {}", e),
            Self::BackendError(e) => write!(f, "Problem with backend: {}", e)
        }
    }
}
impl std::convert::From<teloxide::RequestError> for BotError {
    fn from(e: teloxide::RequestError) -> Self {
        Self::TelegramError(e)
    }
}
impl std::convert::From<io::Error> for BotError {
    fn from(e: io::Error) -> Self {
        Self::BackendError(e)
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
     let bot = teloxide::Bot::new(TOKEN);
     let my_id = bot.get_me().send().await.expect("Failed to get self").user.id;
     teloxide::repl(bot, move |event| async move{
         if let Some(text) = event.update.text() {
             client::save(text).await?;
         };
         if should_reply(&event, my_id) {
             let text = event.update.text().unwrap_or("");
             event.answer(client::generate(text).await?).send().await?;
         };
         Result::<_, BotError>::Ok(())
     }).await;
}

type Cx = teloxide::dispatching::UpdateWithCx<teloxide::Bot, teloxide::types::Message>;
fn should_reply(m: &Cx, my_id: i64) -> bool {
    if m.update.chat.is_private() {
        return true
    }
    else if let Some(text) = m.update.text() {
        if text.starts_with("/pivagen") {
            return true
        }
    }
    else if let Some(from) = m.update.from() {
        if from.id == my_id {
            return true
        }
    }
    if rand::thread_rng().gen_bool(0.15) {
        return true
    }
    false
}
