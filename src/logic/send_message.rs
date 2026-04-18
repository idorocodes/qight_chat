use crate::set_up_connection;

pub async fn send_hello() {
    let client = set_up_connection().await.unwrap();

    client.hello("client-123").await.unwrap();
}