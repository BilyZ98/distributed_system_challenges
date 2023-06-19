use anyhow::Context;
use distributed_system_challenges::*;
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
}

struct EchoNode {
    id: usize,
}

impl Node<(), Payload> for EchoNode {
    fn from_init(
        _state: (),
        _init: Init,
        _tx: std::sync::mpsc::Sender<Event<Payload>>,
    ) -> anyhow::Result<Self> {
        Ok(EchoNode { id: 1 })
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
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to init")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;
            }

            Payload::EchoOk { .. } => {}
        }
        Ok(())
    }
}

pub fn main() -> anyhow::Result<()> {
    main_loop::<_, EchoNode, _, _>(())
}
