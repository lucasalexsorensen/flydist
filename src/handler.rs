use crate::message::{Message, Payload};
use anyhow::{Context, Result};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::{self, Duration};

#[async_trait::async_trait]
pub trait Handler {
    fn init(&mut self, _node_id: String, _node_ids: Vec<String>) {}

    fn handle(&mut self, message: &Message) -> Option<Payload>;

    fn tick_interval(&self) -> Duration {
        Duration::from_secs(1)
    }

    fn tick(&mut self) -> Result<()> {
        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        let stdin = io::stdin();
        let mut lines = BufReader::new(stdin).lines();

        let mut interval = time::interval(self.tick_interval());

        loop {
            tokio::select! {
                maybe_line = lines.next_line() => {
                    if let Some(line) = maybe_line.context("Failed to read line")? {
                        let message: Message = serde_json::from_str(&line).context("Failed to deserialize message")?;
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
                }
                _ = interval.tick() => {
                    self.tick().expect("Failed to tick");
                }
            };
        }
    }
}
