use postgres_derive::ToSql;
use tokio_postgres::{Client, Error};

#[derive(Debug, Clone, Copy, ToSql)]
#[postgres(name = "chat_type")]
pub enum Chat {
    #[postgres(name = "private")]
    Private,
    #[postgres(name = "group")]
    Group,
}
#[derive(Debug, ToSql)]
#[postgres(name = "message_type")]
pub enum MsgType {
    #[postgres(name = "incoming")]
    Incoming,
    #[postgres(name = "outgoing")]
    Outgoing,
}
pub async fn write_message(
    client: &Client,
    msg_type: MsgType,
    chat_type: Chat,
) -> Result<(), Error> {
    client
        .execute(
            "INSERT INTO pivagen_vk VALUES (NOW(), $1, $2)",
            &[&chat_type, &msg_type],
        )
        .await?;
    Ok(())
}
