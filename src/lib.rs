use anyhow::{  Context};
use serde::{Serialize, Deserialize, de::DeserializeOwned };
use std::io::{StdoutLock, Write,  BufRead};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<Payload> { 
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body<Payload>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,

}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitPayload {
    Init(Init),
    InitOk,

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub  struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,

}


// pub trait Payload: Sized {
//     fn extract_init( input: Message<Self>) -> Option<Init>;

//     fn gen_init_ok() -> Self;

// }

pub trait Node<S, Payload> {
    fn from_init(state: S, init: Init) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn step(
        &mut self,
        input: Message<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()>;

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



pub fn main_loop<S, N, P>(init_state : S) -> anyhow::Result<()>
where
    P:  DeserializeOwned  ,
    N: Node<S, P>,
{
    let stdin = std::io::stdin().lock();
    let mut stdin = stdin.lines();
    // let mut inputs  = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<P>>();
    let mut stdout = std::io::stdout().lock();

    let init_msg: Message<InitPayload> = serde_json::from_str(&stdin.next().expect("no init message received")
        .context("failed to read init message from stdin")?,
    )
        .context("init message could not be deserialized")?;

    // let init_msg:  Message<InitPayload> = 
    //     serde_json::from_reader(&mut stdin).context("deserialize init message")?;
    let InitPayload::Init(init) = init_msg.body.payload else {
        panic!("expected init payload");
    };
    // let mut output = serde_json::Serializer::new(stdout);

    // let mut state = EchoNode { id: 0 };
    //
    // let init_msg = inputs
    //     .next()
    //     .expect("no init message received")
    //     .context("init message could not be deserialized")?;
    // let init = P::extract_init(init_msg).expect("init message could not be extracted");
    let mut node: N  = Node::from_init(init_state, init).context("initialization failed")?;

    let reply = Message {
        src: init_msg.dst,
        dst: init_msg.src,
        body: Body {
            id: Some(0),
            in_reply_to: init_msg.body.id,
            payload: InitPayload::InitOk,
        },
    };

    serde_json::to_writer(&mut stdout, &reply).context("serialize response to init")?;
    stdout.write_all(b"\n").context("write trailing newline")?;

    for line in stdin {
        let line = line.context("Maelstrom input from STDIN could not be read")?;
        let input: Message<P> = serde_json::from_str(&line)
            .context("Maelstrom input from STDIN could not be deserialized")?;
        node.step(input, &mut stdout)
            .context("Node step function failed")?;
    }
    Ok(())
}
