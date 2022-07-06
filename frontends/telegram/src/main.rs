use protocol::asyncio::client;
use rand::Rng;
use std::io;
use teloxide::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
enum BotError {
    #[error("Problem with api: {0}")]
    TelegramError(#[from] teloxide::RequestError),

    #[error("Broblem with backend: {0}")]
    BackendError(#[from] io::Error),
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let token = std::env::var("TOKEN").expect("No TOKEN specified");
    let bot = Bot::new(token).auto_send();
    let my_id = bot
        .get_me()
        .await
        .expect("Failed to get self")
        .user
        .id;
    teloxide::repl(bot, move |message: Message, bot: AutoSend<Bot>| async move {
        if let Some(text) = message.text() {
            client::save(text).await?;
        };
        if should_reply(&message, my_id) {
            let text = message.text().unwrap_or("");
            let reply = client::generate(text).await?;
            bot.send_message(message.chat.id, reply).await?;
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
