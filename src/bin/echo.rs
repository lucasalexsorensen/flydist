use anyhow::Result;
use flydist::handler::Handler;
use flydist::message::{Message, Payload};

struct EchoHandler {}

impl Handler for EchoHandler {
    fn handle(&self, message: &Message) -> Payload {
        match &message.body.payload {
            Payload::Echo { echo } => Payload::EchoOk { echo: echo.clone() },
            _ => panic!("Unexpected payload: {:?}", message.body.payload),
        }
    }
}

fn main() -> Result<()> {
    let handler = EchoHandler {};
    handler.run()?;
    Ok(())
}
