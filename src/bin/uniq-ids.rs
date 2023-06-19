use distributed_system_challenges::*;

// use uuid::Uuid;
use anyhow::Context;
use serde::{Deserialize, Serialize};
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
}

struct UniqueNode {
    id: usize,
    node_id: String,
    // ids: Vec<String>,
}

impl Node<(), Payload> for UniqueNode {
    fn from_init(
        _state: (),
        init: Init,
        _tx: std::sync::mpsc::Sender<Event<Payload>>,
    ) -> anyhow::Result<Self> {
        Ok(UniqueNode {
            node_id: init.node_id,
            id: 1,
        })
    }
    fn step(
        &mut self,
        input: Event<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        let Event::Message(input) = input else {
            panic!("unexpected event: {:?}", input);
        };
        match input.body.payload {
            Payload::Generate {} => {
                // let guid = Uuid::new_v4().to_string();
                let guid = format!("{}-{}", self.node_id, self.id);
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::GenerateOk { guid },
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to init")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }

            Payload::GenerateOk { .. } => {}
        }
        Ok(())
    }
}

pub fn main() -> anyhow::Result<()> {
    main_loop::<_, UniqueNode, _, _>(())
}
