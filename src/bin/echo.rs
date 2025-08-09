use anyhow::Result;
use flydist::handler::Handler;
use flydist::message::{Message, Payload};

struct EchoHandler;

impl Handler for EchoHandler {
    fn handle(&mut self, message: &Message) -> Option<Payload> {
        match &message.body.payload {
            Payload::Echo { echo } => Some(Payload::EchoOk { echo: echo.clone() }),
            _ => panic!("Unexpected payload: {:?}", message.body.payload),
        }
    }
}

fn main() -> Result<()> {
    let mut handler = EchoHandler;
    handler.run()?;
    Ok(())
}
