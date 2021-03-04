use rand::Rng;
use serde::Deserialize;
use std::collections::VecDeque;
use std::iter::Iterator;
use std::error::Error;
use protocol::client;

const TOKEN:&str = env!("TOKEN");

#[derive(Deserialize)]
struct Chat
{
    id:i64,
    #[serde(rename = "type")]
    type_:String
}

#[derive(Deserialize)]
struct Sender
{
    #[serde(default)]
    is_bot: bool,
    id: i32,
}

#[derive(Deserialize)]
struct Message 
{
    from: Sender,
    chat: Chat,
    text: Option<String>,
    #[serde(rename = "reply_to_message")]
    reply: Option<Box<Message>>
}

#[derive(Deserialize)]
struct Update 
{
    update_id: i32,
    message: Option<Message>
}
#[derive(Deserialize)]
struct ApiResponse<T>
{
    ok:bool,
    result: T
}

struct LongPoll
{
    offset:i32,
    cache: VecDeque<Update>,
}
impl LongPoll
{
    fn new() -> Self
    {
        Self
        {
            offset:-1,
            cache: VecDeque::new()
        }
    }
    fn get_events(&mut self)
    {
        if cfg!(debug_assertions) {println!("sending request...");}
        let response = match ureq::post(&format!("https://api.telegram.org/bot{}/{}", TOKEN, "getUpdates"))
            .send_form(&[
                ("timeout","25"),                     
                ("offset",&self.offset.to_string()),  
                ("allowed_updates","[\"message\"]")  
                ]
            )
            .into_string()
            {
                Ok(r) => r,
                Err(err) => 
                {
                    eprintln!("{}",err);
                    return
                }
            };
        if cfg!(debug_assertions) {println!("{}", response);}
        let updates = serde_json::from_str::<ApiResponse<Vec<Update>>>(&response).unwrap();
        if !updates.ok
            {return}
        if !updates.result.is_empty() {
            for i in &updates.result
            {
                if i.update_id > self.offset
                    {self.offset = i.update_id}
            }
            self.offset += 1;

            self.cache.extend(updates.result)
        }
    }
}
impl Iterator for LongPoll {
    type Item = Update;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cache.is_empty() {
            self.get_events();
        }
        Some(self.cache.pop_front().unwrap())
    }
}
fn send_message(chat_id: i64, text: &str)
{
    let response = ureq::post(&format!("https://api.telegram.org/bot{}/{}", TOKEN, "sendMessage"))
              .send_form(&[
                    ("chat_id", &chat_id.to_string()),
                    ("text", text)
                    ]
                )
              .into_string();
    if let Err(e) = response {
            eprintln!("{}",e)
        };
}
fn get_my_id() -> Result<i32, ureq::Error> {
    let response = ureq::post(&format!("https://api.telegram.org/bot{}/{}", TOKEN, "getMe"))
              .call()
              .into_string()?;
    let me: ApiResponse<Sender> = serde_json::from_str(&response).unwrap();
    Ok(me.result.id)
}
fn process_message(m: Message, my_id: i32) -> Result<(), Box<dyn Error>> {
    if m.chat.type_ == "channel" {
        return Ok(())
    }
    if let Some(text) = m.text {
        if !m.from.is_bot {
            client::save(&text).unwrap_or(());
        }
        if rand::thread_rng().gen_bool(0.05)
        || m.chat.type_ == "private"
        || text.contains("/pivagen")
        {
            let reply = client::generate(&text).unwrap_or(String::from("Ааеаооеаое"));
            send_message(m.chat.id, &reply)
        }

        else if let Some(reply) = m.reply {
            if reply.from.id == my_id {
                let reply = client::generate(&text).unwrap_or(String::from("Ааеаеоаоео"));
                send_message(m.chat.id, &reply);
            }
        }
    }
    Ok(())    
}
fn main()
{
    let long_poll = LongPoll::new();
    let me = get_my_id().expect("Failed to get self id");

    for e in long_poll {
        if let Some(m) = e.message {
            match process_message(m, me) {
                Ok(()) => {}
                Err(e) => {println!("Error in message processing: {}", e)}
            }
        }
    }
}
