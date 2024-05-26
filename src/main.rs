mod consumer;
mod producer;

use std::io::BufRead;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let uri = "kafka://localhost:19092";
    let topic = "simple-chat";
    let group_id = format!(
        "{}_{}",
        "chat_group",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let username = get_username();

    let consumer = consumer::ChatConsumer::new(uri, topic.to_string(), group_id);
    let producer = producer::ChatProducer::new(uri, topic);
    let producer_clone = producer.await.clone();

    let consumer_handle = tokio::task::spawn(async move {
        consumer.await.consume_message().await;
    });

    let producer_handle = tokio::spawn(async move {
        let stdin = BufReader::new(io::stdin());
        let mut lines = stdin.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            if line.is_empty() {
                continue;
            }
            let chat_message = producer::ChatMessage {
                username: username.clone(),
                message: line.trim().to_string(),
            };

            producer_clone.send_message(chat_message).await;
        }
    });

    let _ = tokio::join!(consumer_handle, producer_handle);
}

fn get_username() -> String {
    println!("Please enter your username: ");
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut username = String::new();
    handle
        .read_line(&mut username)
        .expect("Failed to read username");
    username.trim().to_string()
}
