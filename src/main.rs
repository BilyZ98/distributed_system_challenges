use anyhow::{ bail, Context};
use serde::{Serialize, Deserialize };
use std::io::{StdoutLock, Write};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message { 
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {

    pub node_id: String,
    pub node_ids: Vec<String>,
}



pub trait Node {
    fn step(
        &mut self,
        input: Message,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()>;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo { echo : String },
    EchoOk { echo: String },
    Error { error: String },
    Init { 
        node_id: String,
        node_ids: Vec<String> 
    },
    InitOk,
}


struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn step(
        &mut self,
        input: Message,
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


            Payload::InitOk { .. } =>  {
                bail!("Received InitOk message from {}", input.src);
            }
            Payload::EchoOk { .. } =>  {
            }

            Payload::Error { .. } =>  {
                bail!("Received Error message from {}", input.src);
            }
        }
        Ok(())

    }
    
    // pub fn handle(&mut self, 
    //     input: Message, 
    //     output: &mut serde_json::Serializer<StdoutLock>,
    // ) -> anyhow::Result<()> {
    //     let reply = Message {
    //         src: input.dst,
    //         dst: input.src,
    //         body: Body {
    //             id: Some(self.id),
    //             in_reply_to: input.body.id,
    //             payload: Payload::Echo {
    //                 echo: input.body.payload.echo,
    //             },
    //         },
    //     };
    //     Ok(())
    // }
}

pub fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let inputs  = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();


    let mut stdout = std::io::stdout().lock();
    // let mut output = serde_json::Serializer::new(stdout);

    let mut state = EchoNode { id: 0 };

    for input in inputs {
        let input = input.context("Maelstrom input from STDIN could not be deserialized")?;
        state.step(input, &mut stdout)
            .context("Node step function failed")?;

    }

    Ok(())

}
