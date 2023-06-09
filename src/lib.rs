use anyhow::{ bail, Context};
use serde::{Serialize, Deserialize, de::DeserializeOwned };
use std::io::{StdoutLock, Write};
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
pub struct Init {

    pub node_id: String,
    pub node_ids: Vec<String>,
}



pub trait Node<Payload> {
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



pub fn main_loop<S, Payload>(mut state: S) -> anyhow::Result<()>
where
    S: Node<Payload>,
    Payload: DeserializeOwned,
{
    let stdin = std::io::stdin().lock();
    let inputs  = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<Payload>>();


    let mut stdout = std::io::stdout().lock();
    // let mut output = serde_json::Serializer::new(stdout);

    // let mut state = EchoNode { id: 0 };

    for input in inputs {
        let input = input.context("Maelstrom input from STDIN could not be deserialized")?;
        state.step(input, &mut stdout)
            .context("Node step function failed")?;


    }

    Ok(())

}
