use anyhow::Context;
use distributed_system_challenges::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::{StdoutLock, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    // #[serde(flatten)]
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
    Gossip {
        seen: HashSet<usize>,
    },
    // GossipOk {
    //     seen: HashSet<usize>,
    // },
}

enum InjectedPayload {
    Gossip,
}

struct BroadcastNode {
    id: usize,
    // received_msgs: Vec<usize>,
    messages: Vec<usize>,
    // topology: HashMap<String, Vec<String>>,
    neighbors: Vec<String>,
    node_id: String,
    known: HashMap<String, HashSet<usize>>,

    msg_communicated: HashMap<usize, HashSet<usize>>,
}

impl Node<(), Payload, InjectedPayload> for BroadcastNode {
    fn from_init(
        _state: (),
        init: Init,
        tx: std::sync::mpsc::Sender<Event<Payload, InjectedPayload>>,
    ) -> anyhow::Result<Self> {
        std::thread::spawn(move || {
            // generate gossip events
            // TODO: handle EOF signal
            loop {
                std::thread::sleep(std::time::Duration::from_millis(300));
                if let Err(_) = tx.send(Event::Injected(InjectedPayload::Gossip)) {
                    break;
                }
            }
        });

        Ok(Self {
            node_id: init.node_id,
            id: 1,
            messages: Vec::new(),
            neighbors: Vec::new(),
            known: init
                .node_ids
                .into_iter()
                .map(|nid| (nid, HashSet::new()))
                .collect(),
            msg_communicated: HashMap::new(),
        })
    }
    fn step(
        &mut self,
        input: Event<Payload, InjectedPayload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match input {
            Event::EOF => {}

            Event::Injected(payload) => match payload {
                InjectedPayload::Gossip => {
                    for n in &self.neighbors {
                        let known_to_n = &self.known[n];
                        let (already_known, mut notify_of): (HashSet<_>, HashSet<_>) = self
                            .messages
                            .iter()
                            .copied()
                            .partition(|m| !known_to_n.contains(m));
                        // if we know that n knows m, we don't tell n that _we_ know m,
                        // so n will send us m for all eternity, so, we include a  copule
                        // of extra m's messages so they gradually know all the things
                        // that we know without sending lots of extra stuff each time
                        // set pecentage of extra messages to 10% to avoid excessive overhead
                        eprintln!("notify of {}/{}", notify_of.len(), self.messages.len());

                        notify_of.extend(already_known.iter().filter(|_| {
                            rand::thread_rng().gen_ratio(
                                10.min(already_known.len()) as u32,
                                already_known.len() as u32,
                            )
                        }));
                        Message {
                            src: self.node_id.clone(),
                            dst: n.clone(),
                            body: Body {
                                id: None,
                                in_reply_to: None,
                                payload: Payload::Gossip { seen: notify_of },
                            },
                        }
                        .send(&mut *output)
                        .with_context(|| format!("send gossip to {}", n))?;
                        self.id += 1;
                    }
                }
            },

            Event::Message(input) => {
                let mut reply = input.into_reply(Some(&mut self.id));
                match reply.body.payload {
                    Payload::Gossip { seen } => {
                        self.known
                            .get_mut(&reply.dst)
                            .expect("dst node known")
                            .extend(seen.iter().copied());
                        self.messages.extend(seen);
                    }
                    Payload::Broadcast { message } => {
                        self.messages.push(message);
                        reply.body.payload = Payload::BroadcastOk;
                        reply.send(&mut *output).context("reply  to broadcast")?;
                        self.id += 1;
                    }

                    Payload::BroadcastOk {} => {}

                    Payload::Read {} => {
                        reply.body.payload = Payload::ReadOk {
                            messages: self.messages.clone(),
                        };
                        reply.send(&mut *output).context("reply to read")?;
                        self.id += 1;
                    }

                    Payload::ReadOk { .. } => {}

                    Payload::Topology { mut topology } => {
                        // self.topology = topology;
                        self.neighbors = topology
                            .remove(&self.node_id)
                            .unwrap_or_else(|| panic!("no tpoology for node {}", self.node_id));
                        reply.body.payload = Payload::TopologyOk;
                        serde_json::to_writer(&mut *output, &reply)
                            .context("serialize response to topology")?;
                        output.write_all(b"\n").context("write trailing newline")?;
                        self.id += 1;
                    }

                    Payload::TopologyOk {} => {}
                }
            }
        }
        Ok(())
    }
}

pub fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _, _>(())
}
