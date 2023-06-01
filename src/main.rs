use anyhow::{ Context};
use serde::{Serialize, de::Deserialize, DeserializeOwned};
use std::io::{StdoutLock, Write};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<Payload> { 
    pub src: String,
    #[serde(rename = "dst")]
    pub dst: String,
    pub body: Body<Payload>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
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
    pub fn step(
        &mut self,
        input: Message<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()>;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
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
};

impl EchoNode {
    pub fn step(
        &mut self,
        input: Message,
        output: &mut serde_json::Serializer<StdoutLock>,
    ) -> anyhow::Result<()> {
        let reply = Message {
            src: input.dst,
            dst: input.src,
            body: Body {
                id: Some(self.id),
                in_reply_to: input.body.id,
                payload: Payload::EchoOk { echo},
            },
        };
        self.id += 1;
        Ok(())
    }
    )
    pub fn handle(&mut self, 
        input: Message, 
        output: &mut serde_json::Serializer<StdoutLock>,
    ) -> anyhow::Result<()> {
        let reply = Message {
            src: input.dst,
            dst: input.src,
            body: Body {
                id: Some(self.id),
                in_reply_to: input.body.id,
                payload: Payload::Echo {
                    echo: input.body.payload.echo,
                },
            },
        }
        Ok(())
    }
}

pub fn main_loop<S, Payload>(mut state: S) -> anyhow::Result<()>
where 
    S: Node<Payload>,
    Payload: Deserialize{
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();

    let input = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();


    for input in inputs {
        let input = input?;
    }

    println!("Hello, world!");
}
