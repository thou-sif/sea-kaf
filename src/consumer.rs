use sea_streamer::export::futures::StreamExt;
use sea_streamer::kafka::AutoOffsetReset;
use sea_streamer::SeaConsumerOptions;
use sea_streamer::{
    Buffer, Consumer, ConsumerGroup, ConsumerMode, ConsumerOptions, Message, SeaConsumer,
    SeaStreamer, StreamKey, Streamer, StreamerUri,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub username: String,
    pub message: String,
}

pub struct ChatConsumer {
    consumer: SeaConsumer,
}

impl ChatConsumer {
    pub async fn new(broker: &str, topic: String, group_id: String) -> Self {
        let uri = StreamerUri::from_str(broker).expect("Invalid URL");
        let streamer = SeaStreamer::connect(uri, Default::default())
            .await
            .expect("Failed tp connect to streamer");
        let topic = StreamKey::new(topic).expect("Invalid Topic");
        let mut consumer_options = SeaConsumerOptions::new(ConsumerMode::LoadBalanced);
        consumer_options.set_kafka_consumer_options(|options| {
            options.set_group_id(ConsumerGroup::new(group_id));
            options.set_auto_offset_reset(AutoOffsetReset::Earliest);
        });
        let consumer = streamer
            .create_consumer(&[topic], consumer_options)
            .await
            .unwrap();

        ChatConsumer { consumer }
    }

    pub async fn consume_message(&mut self) {
        let mut stream = self.consumer.stream();

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => {
                    let payload = message.message();
                    match serde_json::from_str::<ChatMessage>(payload.as_str().unwrap()) {
                        Ok(chat_message) => {
                            println!("{}: {}", chat_message.username, chat_message.message);
                        }
                        Err(error) => {
                            eprintln!("Failed to deserialize message: {}", error);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Failed to receive message: {}", error);
                }
            }
        }
    }
}
