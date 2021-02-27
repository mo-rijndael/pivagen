use serde::Deserialize;
use std::collections::VecDeque;
use std::error::Error;
use std::iter::Iterator;
use crate::{TOKEN};

macro_rules! gen_vk_call {
    ($vis:vis $name:ident $method:literal $( : $( $argument:ident=$type:ty ),+ )? => $ret:ty) => {
        $vis fn $name($($( $argument: $type ),+)?) -> Result<$ret, Box<dyn Error>> {
            let response = ureq::post(concat!("https://api.vk.com/method/", $method))
                .send_form(&[
                    ("access_token", TOKEN),
                    ("v", "5.95"),
               $($( (stringify!($argument), &$argument.to_string()) ),+ )?
                ])?;
            let text = response.into_string()?;
            if cfg!(debug_assertions) { println!("{}", &text) }
            let data: $ret = serde_json::from_str(&text)?;
            Ok(data)
        }
        
    }
}

#[derive(Deserialize)]
struct Object {
    object: Message,
}

#[derive(Deserialize)]
pub struct ReplyMessage {
    pub from_id: i32
}
#[derive(Deserialize)]
pub struct Message {
    peer_id: i32,
    from_id: i32,
    pub text: String,
    pub reply_message: Option<ReplyMessage>,
}

#[derive(Deserialize)]
pub struct VKResponse<T> {
    pub response: T,
}
#[derive(Deserialize)]
pub struct Group{
    pub id: i32,
}
#[derive(Deserialize)]
pub struct LongPoll {
    server: Box<str>,
    key: Box<str>,
    ts: Box<str>,
    #[serde(skip)]
    cache: VecDeque<Message>,
    #[serde(skip)]
    group_id: i32,
}
#[derive(Deserialize)]
struct LongPollOk{
    ts: Box<str>,
    updates: Vec<Object>,
}
#[derive(Deserialize)]
struct LongPollError {
    failed: u8,
    ts: Option<Box<str>>
}
#[derive(Deserialize)]
#[serde(untagged)]
enum LongPollResult {
    Events(LongPollOk),
    Error(LongPollError),
}
impl LongPoll {
    pub fn new(id: i32) -> Result<LongPoll, Box<dyn Error>> {
        let mut res = get_longpoll(id)?.response;
        res.group_id = id;
        Ok(res)
        
    }
    fn get_events(&mut self) -> Result<(), Box<dyn Error>>{
        let response = ureq::post(&self.server)
            .send_form(&[
                ("act", "a_check"),
                ("key", &self.key),
                ("ts", &self.ts),
                ("wait", "25")
                ]
            );
        if let Ok(response) = response {
            let text = response.into_string()?;
            if cfg!(debug_assertions) {println!("{}",&text)}
            match serde_json::from_str::<LongPollResult>(&text)? {
                LongPollResult::Events(ok) => {
                    self.ts = ok.ts;
                    self.cache.extend(ok.updates.into_iter().map(|obj| obj.object))
                }
                LongPollResult::Error(err) => {
                    println!("got longpoll error {}", err.failed);
                    let new_longpoll = Self::new(self.group_id)?;
                    match err.failed {
                        1 => {self.ts = err.ts.unwrap()}
                        2 => {self.key = new_longpoll.key}
                        3 => {self.key = new_longpoll.key;
                              self.ts = new_longpoll.ts}
                        _ => {*self = new_longpoll;}
                    }
                }
            }
        }
        Ok(())
    }
}
impl Iterator for LongPoll {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cache.is_empty() {
            self.get_events().unwrap_or_else(|_|{});
        }
        self.cache.pop_front()
    }
}

impl Message {
    pub fn reply(&self, text: &str){
        let random_id:i64 = rand::random();
        let response = ureq::post("https://api.vk.com/method/messages.send")
            .send_form(&[
                ("access_token", TOKEN),
                ("v", "5.95"),
                ("random_id", &random_id.to_string()),
                ("peer_id", &self.peer_id.to_string()),
                ("message", text)
                ]
            );
        if response.is_err() {
            eprintln!("ERROR {}", response.unwrap_err())
        }
    }
    pub fn is_private(&self) -> bool {
        self.peer_id < 2_000_000_000
    }
    pub fn from_user(&self) -> bool {
        self.from_id > 0
    }
}
gen_vk_call!{pub get_me "groups.getById" => VKResponse<[Group; 1]>}
gen_vk_call!{get_longpoll "groups.getLongPollServer": group_id=i32 => VKResponse<LongPoll>}
