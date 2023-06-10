use distributed_system_challenges::*;

use uuid::Uuid;
use anyhow::{ bail, Context};
use serde::{Serialize, Deserialize };
use std::io::{StdoutLock, Write};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Generate,
    GenerateOk { 
        #[serde(rename = "id")]
        guid: String, 
    },
    Init { 
        node_id: String,
        node_ids: Vec<String> 
    },
    InitOk,
}






struct UniqueNode {
    id: usize,
    node_id: String,

    // ids: Vec<String>,
}

impl Node<Payload> for UniqueNode {
    fn step(
        &mut self,
        input: Message<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } =>  {
                let reply  = Message { 
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply).
                    context("serialize response to init")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }

            Payload::Generate {  } =>  {
                let guid = Uuid::new_v4().to_string();
                let guid = format!("{}-{}", self.node_id, self.id)
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::GenerateOk { guid, },
                    },
                };
                serde_json::to_writer(&mut *output, &reply).
                    context("serialize response to init")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }


            Payload::InitOk { .. } =>  {
                bail!("Received InitOk message from {}", input.src);
            }
            Payload::GenerateOk { .. } =>  {
            }


        }
        Ok(())

    }
}


 
pub fn main() -> anyhow::Result<()> {
    main_loop(UniqueNode { id: 0 })
}




