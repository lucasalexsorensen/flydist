use anyhow::Result;
use flydist::handler::Handler;
use flydist::message::{Message, Payload};

struct GenerateHandler;

impl Handler for GenerateHandler {
    fn handle(&mut self, message: &Message) -> Option<Payload> {
        match &message.body.payload {
            Payload::Generate => Some(Payload::GenerateOk {
                id: uuid::Uuid::new_v4().to_string(),
            }),
            _ => panic!("Unexpected payload: {:?}", message.body.payload),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut handler = GenerateHandler;
    handler.run().await?;
    Ok(())
}
