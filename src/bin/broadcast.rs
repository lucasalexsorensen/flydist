use std::collections::HashSet;

use anyhow::Result;
use flydist::handler::Handler;
use flydist::message::{Message, Payload};

#[derive(Default)]
struct BroadcastHandler {
    node_id: Option<String>,
    neighbours: Option<Vec<String>>,
    messages: HashSet<usize>,
    new_messages: HashSet<usize>,
}

#[async_trait::async_trait]
impl Handler for BroadcastHandler {
    fn init(&mut self, node_id: String, _node_ids: Vec<String>) {
        self.node_id = Some(node_id);
    }

    fn tick_interval(&self) -> tokio::time::Duration {
        tokio::time::Duration::from_millis(175)
    }

    fn handle(&mut self, message: &Message) -> Option<Payload> {
        match &message.body.payload {
            Payload::Topology { topology } => {
                self.neighbours = Some(
                    topology
                        .get(self.node_id.as_ref().expect("Node ID not set"))
                        .expect("Topology not set")
                        .clone(),
                );
                Some(Payload::TopologyOk)
            }
            Payload::Read => Some(Payload::ReadOk {
                messages: self.messages.iter().cloned().collect(),
            }),
            Payload::Broadcast { message: k } => {
                if !self.messages.contains(k) {
                    self.new_messages.insert(*k);
                }
                self.messages.insert(*k);

                Some(Payload::BroadcastOk)
            }
            Payload::BroadcastOk => None,
            Payload::Gossip { messages } => {
                let diff = self.messages.difference(messages).cloned().collect();
                self.messages.extend(messages.iter().cloned());
                Some(Payload::GossipOk { diff })
            }
            Payload::GossipOk { diff } => {
                self.messages.extend(diff.iter().cloned());
                None
            }
            _ => panic!("Unexpected payload: {:?}", message.body.payload),
        }
    }

    /// On every tick, each node will gossip with its neighbours.
    /// They send their entire set of messages, and receive a diff of what they're missing from the neighbour.
    fn tick(&mut self) -> Result<()> {
        if let Some(neighbours) = self.neighbours.as_ref()
            && let Some(node_id) = self.node_id.as_ref()
        {
            neighbours.iter().for_each(|neighbour| {
                let src = node_id.clone();
                let dst = neighbour.clone();
                let msg = Message::new(
                    src,
                    dst,
                    Payload::Gossip {
                        messages: self.new_messages.clone(),
                    },
                );
                println!("{}", msg);
            });
            self.new_messages.clear();
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut handler = BroadcastHandler::default();
    handler.run().await?;
    Ok(())
}
