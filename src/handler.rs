use crate::message::{Message, Payload};
use anyhow::{Context, Result};

pub trait Handler {
    fn handle(&self, message: &Message) -> Payload;

    fn run(&self) -> Result<()> {
        let stdin = std::io::stdin().lock();
        let messages = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

        for message in messages {
            let message = message.context("Failed to deserialize message")?;

            let response_payload = match &message.body.payload {
                Payload::Init {
                    node_id: _,
                    node_ids: _,
                } => Payload::InitOk {},
                _ => self.handle(&message),
            };

            let response = message.into_response(response_payload);

            println!("{}", serde_json::to_string(&response)?);
        }

        Ok(())
    }
}
