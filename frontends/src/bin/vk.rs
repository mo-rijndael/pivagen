use serde::Deserialize;
use rand::Rng;
use std::collections::VecDeque;
use std::iter::Iterator;
use frontends::protocol;

const TOKEN: &str = env!("TOKEN");
const GID: &str = env!("GROUP_ID");

#[derive(Deserialize)]
struct Object {
    object: Message,
}

#[derive(Deserialize)]
struct ReplyMessage {
    from_id: i32
}
#[derive(Deserialize)]
struct Message {
    peer_id: i32,
    from_id: i32,
    text: String,
    reply_message: Option<ReplyMessage>,
}

#[derive(Deserialize)]
struct VKResponse<T> {
    response: T,
}

#[derive(Deserialize)]
struct LongPoll {
    server: Box<str>,
    key: Box<str>,
    ts: Box<str>,
    #[serde(skip)]
    cache: VecDeque<Message>,
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
    fn new() -> LongPoll{
        print!("getting new LongPoll...");
        let res = ureq::post("https://api.vk.com/method/groups.getLongPollServer")
            .send_form(&[
                ("access_token",TOKEN),
                ("group_id",GID),
                ("v","5.95")
                ]    
            );
        let res = &res.into_string().unwrap();
        println!("{}", res);
        let r:VKResponse<LongPoll> = serde_json::from_str(res).unwrap();
        r.response
        
    }
    fn get_events(&mut self) {
        let response = ureq::post(&self.server)
            .send_form(&[
                ("act", "a_check"),
                ("key", &self.key),
                ("ts", &self.ts),
                ("wait", "25")
                ]
            );
        if response.ok() {
            let text = response.into_string().unwrap();
            if cfg!(debug_assertions) {println!("{}",&text)}
            match serde_json::from_str::<LongPollResult>(&text).unwrap() {
                LongPollResult::Events(ok) => {
                    self.ts = ok.ts;
                    self.cache.extend(ok.updates.into_iter().map(|obj| obj.object))
                }
                LongPollResult::Error(err) => {
                    println!("got longpoll error {}", err.failed);
                    let new_longpoll = Self::new();
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
    }
}
impl Iterator for LongPoll {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cache.is_empty() {
            self.get_events();
        }
        Some(self.cache.pop_front().unwrap())
    }
}


fn send_message(peer_id: i32, text: &str){
    println!("sending: \"{}\" to:{} ", text, peer_id);
    let random_id:i64 = rand::random();
    let response = ureq::post("https://api.vk.com/method/messages.send")
        .send_form(&[
            ("access_token", TOKEN),
            ("v", "5.95"),
            ("random_id", &random_id.to_string()),
            ("peer_id", &peer_id.to_string()),
            ("message", text)
            ]
        );
    if response.error() {
        eprintln!("ERROR {}", response.status_text())
    }
}


fn main() {
    let mut rand = rand::thread_rng();
    let group_id: i32 = std::str::FromStr::from_str(GID).unwrap();
    let longpoll = LongPoll::new();

    for m in longpoll {
        println!("chat:{}, sender:{}, text:\"{}\"", m.peer_id, m.from_id, m.text);
        if m.from_id > 0 {
            protocol::save(&m.text).unwrap();
        }
        
        if rand.gen_bool(0.05)
        || m.peer_id & 2_000_000_000 != 2_000_000_000
        || (m.from_id == -176_707_471 && rand.gen_bool(0.1))
        || m.text.contains(&format!("[club{}|", GID))
        || (m.reply_message.is_some() && m.reply_message.unwrap().from_id == -group_id)
        {
            send_message(m.peer_id, &protocol::generate(&m.text).unwrap())
        }
    }
}
