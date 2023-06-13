use distributed_system_challenges::*;
use anyhow::{  Context};
use serde::{Serialize, Deserialize };
use std::io::{StdoutLock, Write};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {

    Broadcast { message: usize },
    BroadcastOk,
    Read ,
    ReadOk { messages: Vec<usize> },
    // #[serde(flatten)]
    Topology { topology: HashMap<String, Vec<String>> },
    TopologyOk,
}

struct BroadcastNode {
    id: usize,
    received_msgs: Vec<usize>,
    topology: HashMap<String, Vec<String>>,
    node_id: String,
}


impl Node<(), Payload> for BroadcastNode {
    fn from_init(_state: (), init: Init) -> anyhow::Result<Self> {
        Ok(BroadcastNode {id: 1, received_msgs: Vec::new(), topology: HashMap::new(), node_id: init.node_id, })
    }
    fn step(
        &mut self,
        input: Message<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        let mut reply =  input.into_reply(Some(&mut self.id));
        match reply.body.payload {
            Payload::Broadcast { message } =>  {
                self.received_msgs.push(message);
                reply.body.payload = Payload::BroadcastOk;
                serde_json::to_writer(&mut *output, &reply).
                    context("serialize response to broadcast")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }

            
            Payload::BroadcastOk {  } =>  {
            }

            Payload::Read { } =>  {
                reply.body.payload = Payload::ReadOk { messages: self.received_msgs.clone() };
                serde_json::to_writer(&mut *output, &reply).
                    context("serialize response to Read")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }

            Payload::ReadOk { ..  } =>  {
            }

            Payload::Topology { topology } =>  {
                self.topology = topology;
                reply.body.payload = Payload::TopologyOk;
                serde_json::to_writer(&mut *output, &reply).
                    context("serialize response to topology")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }

            Payload::TopologyOk {   } =>  {

            }

        }
        Ok(())

    }

}

pub fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _>(())

}
