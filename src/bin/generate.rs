use anyhow::Result;
use flydist::handler::Handler;
use flydist::message::{Message, Payload};

struct EchoHandler {}

impl Handler for EchoHandler {
    fn handle(&self, message: &Message) -> Payload {
        match &message.body.payload {
            Payload::Generate {} => Payload::GenerateOk {
                id: uuid::Uuid::new_v4().to_string(),
            },
            _ => panic!("Unexpected payload: {:?}", message.body.payload),
        }
    }
}

fn main() -> Result<()> {
    let handler = EchoHandler {};
    handler.run()?;
    Ok(())
}
