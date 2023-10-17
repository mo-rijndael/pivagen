use crate::TOKEN;
use serde::Deserialize;
use std::error::Error;
use std::iter::Iterator;

macro_rules! gen_vk_call {
    ($vis:vis $name:ident $method:literal $( : $( $argument:ident=$type:ty ),+ )? => $ret:ty) => {
        $vis async fn $name($($( $argument: $type ),+)?) -> Result<$ret, Box<dyn Error>> {
            let client = reqwest::Client::new();
            let response = client.post(concat!("https://api.vk.com/method/", $method))
                .form(&[
                    ("v", "5.131"),
                    ("access_token", &TOKEN),
               $($( (stringify!($argument), &$argument.to_string()) ),+ )?
                ])
                .send().await?;
            let text = response.text().await?;
            if cfg!(debug_assertions) { println!("{}", &text) }
            let data: $ret = serde_json::from_str(&text)?;
            Ok(data)
        }

    }
}

#[derive(Deserialize)]
struct Object {
    object: MessageInline,
}

#[derive(Deserialize)]
struct MessageInline {
    message: Message,
}

#[derive(Deserialize)]
pub struct ReplyMessage {
    pub from_id: i32,
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
pub struct Group {
    pub id: i32,
}
#[derive(Deserialize)]
pub struct LongPoll {
    server: String,
    key: String,
    ts: String,
    #[serde(skip)]
    group_id: i32,
}
#[derive(Deserialize)]
struct LongPollOk {
    ts: String,
    updates: Vec<Object>,
}
#[derive(Deserialize)]
struct LongPollError {
    failed: u8,
    ts: Option<String>,
}
#[derive(Deserialize)]
#[serde(untagged)]
enum LongPollResult {
    Events(LongPollOk),
    Error(LongPollError),
}
impl LongPoll {
    pub async fn new(id: i32) -> Result<LongPoll, Box<dyn Error>> {
        let mut res = get_longpoll(id).await?.response;
        res.group_id = id;
        Ok(res)
    }
    pub async fn get_events(
        &mut self,
        client: &reqwest::Client,
    ) -> Result<Vec<Message>, Box<dyn Error>> {
        let response = client
            .post(&self.server)
            .form(&[
                ("act", "a_check"),
                ("key", &self.key),
                ("ts", &self.ts),
                ("wait", "25"),
            ])
            .send()
            .await?;
        let text = response.text().await?;
        if cfg!(debug_assertions) {
            println!("{}", &text)
        }
        match serde_json::from_str::<LongPollResult>(&text)? {
            LongPollResult::Events(ok) => {
                self.ts = ok.ts;
                return Ok(ok.updates.into_iter().map(|o| o.object.message).collect());
            }
            LongPollResult::Error(err) => {
                println!("got longpoll error {}", err.failed);
                let new_longpoll = Self::new(self.group_id).await?;
                match err.failed {
                    1 => self.ts = err.ts.unwrap(),
                    2 => self.key = new_longpoll.key,
                    3 => {
                        self.key = new_longpoll.key;
                        self.ts = new_longpoll.ts;
                    }
                    _ => {
                        *self = new_longpoll;
                    }
                }
                Ok(vec![])
            }
        }
    }
}
impl Message {
    pub async fn reply(&self, text: &str, client: &reqwest::Client) {
        let random_id: i64 = rand::random();
        let response = client
            .post("https://api.vk.com/method/messages.send")
            .form(&[
                ("v", "5.131"),
                ("access_token", &TOKEN),
                ("random_id", &random_id.to_string()),
                ("peer_id", &self.peer_id.to_string()),
                ("message", text),
            ])
            .send()
            .await;
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
gen_vk_call! {pub get_me "groups.getById" => VKResponse<[Group; 1]>}
gen_vk_call! {get_longpoll "groups.getLongPollServer": group_id=i32 => VKResponse<LongPoll>}
