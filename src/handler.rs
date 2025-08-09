use crate::message::{Message, Payload};
use anyhow::{Context, Result};

pub trait Handler {
    fn init(&mut self, _node_id: String, _node_ids: Vec<String>) {}

    fn handle(&mut self, message: &Message) -> Option<Payload>;

    fn run(&mut self) -> Result<()> {
        let stdin = std::io::stdin().lock();
        let messages = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

        for message in messages {
            let message = message.context("Failed to deserialize message")?;

            let response_payload = match &message.body.payload {
                Payload::Init { node_id, node_ids } => {
                    self.init(node_id.clone(), node_ids.clone());
                    Some(Payload::InitOk)
                }
                _ => self.handle(&message),
            };

            if let Some(response_payload) = response_payload {
                let response = message.into_response(response_payload);
                println!("{}", response);
            }
        }

        Ok(())
    }
}
