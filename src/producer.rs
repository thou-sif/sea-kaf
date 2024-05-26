use sea_streamer::{Producer, SeaProducer, SeaStreamer, StreamKey, Streamer, StreamerUri};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub username: String,
    pub message: String,
}

#[derive(Clone)]
pub struct ChatProducer {
    producer: SeaProducer,
    topic: StreamKey,
}

impl ChatProducer {
    pub async fn new(brokers: &str, topic: &str) -> Self {
        let uri = StreamerUri::from_str(brokers).expect("Invalid broker url");
        let streamer = SeaStreamer::connect(uri, Default::default())
            .await
            .expect("Failed to connect to streamer");
        let topic = StreamKey::new(topic).expect("Invalid topic");
        let producer = streamer
            .create_producer(topic.clone(), Default::default())
            .await
            .expect("Failed to create producer");

        ChatProducer { producer, topic }
    }

    pub async fn send_message(&self, message: ChatMessage) {
        let payload = serde_json::to_string(&message).expect("Failed to serialize message");
        self.producer.send(payload).expect("Failed to send message");
    }
}
