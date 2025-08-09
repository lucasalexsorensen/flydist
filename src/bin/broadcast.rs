use std::collections::{HashMap, HashSet};

use anyhow::Result;
use flydist::handler::Handler;
use flydist::message::{Message, Payload};

#[derive(Default)]
struct BroadcastHandler {
    node_id: Option<String>,
    messages: HashSet<usize>,
    topology: HashMap<String, Vec<String>>,
}

impl Handler for BroadcastHandler {
    fn init(&mut self, node_id: String, _node_ids: Vec<String>) {
        self.node_id = Some(node_id);
    }

    fn handle(&mut self, message: &Message) -> Option<Payload> {
        match &message.body.payload {
            Payload::Topology { topology } => {
                self.topology = topology.clone();
                Some(Payload::TopologyOk)
            }
            Payload::Broadcast { message: k } => {
                if !self.messages.contains(k) {
                    let self_id = self.node_id.as_ref().expect("Node ID not set");
                    let neighbours = self.topology.get(self_id).expect("Topology not set");

                    neighbours.iter().for_each(|neighbour| {
                        if neighbour != &message.src {
                            println!(
                                "{}",
                                Message::new(
                                    self_id.clone(),
                                    neighbour.clone(),
                                    Payload::Broadcast { message: *k },
                                )
                            );
                        }
                    });
                    self.messages.insert(*k);
                }

                Some(Payload::BroadcastOk)
            }
            Payload::Read => Some(Payload::ReadOk {
                messages: self.messages.iter().cloned().collect(),
            }),
            Payload::BroadcastOk => None,
            _ => panic!("Unexpected payload: {:?}", message.body.payload),
        }
    }
}

fn main() -> Result<()> {
    let mut handler = BroadcastHandler::default();
    handler.run()?;
    Ok(())
}
