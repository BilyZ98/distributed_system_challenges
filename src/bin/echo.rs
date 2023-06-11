use distributed_system_challenges::*;
use anyhow::{  Context};
use serde::{Serialize, Deserialize };
use std::io::{StdoutLock, Write};
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Message { 
//     pub src: String,
//     #[serde(rename = "dest")]
//     pub dst: String,
//     pub body: Body,
// }



// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Body {
//     #[serde(rename = "msg_id")]
//     pub id: Option<usize>,
//     pub in_reply_to: Option<usize>,
//     #[serde(flatten)]
//     pub payload: Payload,

// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Init {

//     pub node_id: String,
//     pub node_ids: Vec<String>,
// }



// pub trait Node {
//     fn step(
//         &mut self,
//         input: Message,
//         output: &mut StdoutLock,
//     ) -> anyhow::Result<()>;
// }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo { echo : String },
    EchoOk { echo: String },
}

// impl distributed_system_challenges::Payload for Payload {
//     fn extract_init( input: Self) -> Option<Init> {
//         let Payload::Init(init) = input else {
//             return None;
//         };
//         Some(init)
//     }

//     fn gen_init_ok() -> Self {
//         Payload::InitOk
//     }
// }

struct EchoNode {
    id: usize,
}

impl Node<(), Payload> for EchoNode {
    fn from_init(_state: (), _init: Init) -> anyhow::Result<Self> {
        Ok(EchoNode {id: 1 })
    }
    fn step(
        &mut self,
        input: Message<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } =>  {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo},
                    },
                };
                serde_json::to_writer(&mut *output, &reply).
                    context("serialize response to init")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }

            
            Payload::EchoOk { .. } =>  {
            }

            // Payload::Error { .. } =>  {
            //     bail!("Received Error message from {}", input.src);
            // }
        }
        Ok(())

    }

}

pub fn main() -> anyhow::Result<()> {
    main_loop::<_, EchoNode, _>(())

}
