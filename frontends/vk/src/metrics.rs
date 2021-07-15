use tokio_postgres::{Client, Error};

#[derive(Clone, Copy)]
pub enum Chat {
    Private,
    Group,
}
impl Chat {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::Group => "group",
        }
    }
}
pub enum MsgType {
    Incoming,
    Outgoing,
}
impl MsgType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Incoming => "incoming",
            Self::Outgoing => "outgoing",
        }
    }
}
pub async fn write_message(
    client: &Client,
    msg_type: MsgType,
    chat_type: Chat,
) -> Result<(), Error> {
    client
        .execute(
            "INSERT INTO pivagen_vk VALUES (NOW(), '$1', '$2')",
            &[&msg_type.as_str(), &chat_type.as_str()],
        )
        .await?;
    Ok(())
}
