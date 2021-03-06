use protocol::client;
use rand::Rng;
use std::error::Error;
mod api;
use api::*;

const TOKEN: &str = env!("TOKEN");

fn should_reply(message: &Message, my_id: i32) -> bool {
    message.is_private()
        || message.text.contains(&format!("[club{}|", my_id))
        || (message.reply_message.is_some()
            && message.reply_message.as_ref().unwrap().from_id == -my_id)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rand = rand::thread_rng();
    let group_id = get_me()?.response[0].id;
    let longpoll = LongPoll::new(group_id)?;
    let backend_unavailable = String::from("Ааеаоаооаа");

    for m in longpoll {
        if m.from_user() {
            let res = client::save(&m.text);
            if let Err(e) = res {
                println!("Failed to save: {}", e);
            }
        }

        if should_reply(&m, group_id) {
            let reply = match client::generate(&m.text) {
                Ok(reply) => reply,
                Err(_) => backend_unavailable.clone(),
            };
            m.reply(&reply)
        } else if rand.gen_bool(0.05) {
            if let Ok(reply) = client::generate(&m.text) {
                m.reply(&reply)
            }
        }
    }
    Ok(())
}
