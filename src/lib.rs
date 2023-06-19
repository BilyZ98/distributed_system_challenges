use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{BufRead, StdoutLock, Write};
use std::sync;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<Payload> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body<Payload>,
}

#[derive(Debug, Clone)]
pub enum Event<Payload, InjectedPayload = ()> {
    Message(Message<Payload>),
    Injected(InjectedPayload),
    EOF,
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
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

// pub trait Payload: Sized {
//     fn extract_init( input: Message<Self>) -> Option<Init>;

//     fn gen_init_ok() -> Self;

// }

pub trait Node<S, Payload, InjectedPayload = ()> {
    fn from_init(
        state: S,
        init: Init,
        inject: std::sync::mpsc::Sender<Event<Payload, InjectedPayload>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn step(
        &mut self,
        input: Event<Payload, InjectedPayload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()>;
}

impl<Payload> Message<Payload> {
    pub fn into_reply(
        self,
        id: Option<&mut usize>,
    ) -> Self {
        Message {
            src: self.dst,
            dst: self.src,
            body: Body {
                id: id.map(|id| {
                    let mid = *id;
                    *id += 1;
                    mid
                }),
                in_reply_to: self.body.id,
                payload: self.body.payload,
            },
        }
    }

    pub fn send(
        &self,
        mut output: impl Write,
    ) -> anyhow::Result<()>
    where
        Payload: Serialize,
    {
        serde_json::to_writer(&mut output, self).context("serialize response msg ")?;
        output.write_all(b"\n").context("write trailing newline")?;
        Ok(())
    }
}

pub fn main_loop<S, N, P, IP>(init_state: S) -> anyhow::Result<()>
where
    P: DeserializeOwned + Send + 'static,
    N: Node<S, P, IP>,
    IP: Send + 'static,
{
    let stdin = std::io::stdin().lock();
    let mut stdin = stdin.lines();
    // let mut inputs  = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<P>>();
    let mut stdout = std::io::stdout().lock();

    let init_msg: Message<InitPayload> = serde_json::from_str(
        &stdin
            .next()
            .expect("no init message received")
            .context("failed to read init message from stdin")?,
    )
    .context("init message could not be deserialized")?;

    // let init_msg:  Message<InitPayload> =
    //     serde_json::from_reader(&mut stdin).context("deserialize init message")?;
    let InitPayload::Init(init) = init_msg.body.payload else {
        panic!("expected init payload");
    };
    // let mut output = serde_json::Serializer::new(stdout);

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

    let (tx, rx) = std::sync::mpsc::channel();
    let mut node: N =
        Node::from_init(init_state, init, tx.clone()).context("initialization failed")?;
    drop(stdin);
    // let stdin_tx = tx.clone();
    let jh = std::thread::spawn(move || {
        let stdin = std::io::stdin().lock();
        for line in stdin.lines() {
            let line = line.context("Maelstrom input from STDIN could not be read")?;
            let input: Message<P> = serde_json::from_str(&line)
                .context("Maelstrom input from STDIN could not be deserialized")?;

            if let Err(_) = tx.send(Event::Message(input)) {
                return Ok::<_, anyhow::Error>(());
            }
        }
        let _ = tx.send(Event::EOF);
        Ok(())
    });

    for input in rx {
        node.step(input, &mut stdout)
            .context("Node step function failed")?;
    }

    jh.join()
        .expect("stdin thread panicked")
        .context("stdin thread failed'd")?;

    Ok(())
}
