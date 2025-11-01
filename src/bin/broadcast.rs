use anyhow::Context;
use gossip_glomers::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{StdoutLock, Write},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

struct BroadcastNode {
    node: String,
    id: usize,
    messages: Vec<usize>,
}

impl Node<(), Payload> for BroadcastNode {
    fn from_init(_state: (), init: gossip_glomers::Init) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(BroadcastNode {
            id: 1,
            node: init.node_id,
            messages: Vec::new(),
        })
    }
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        let reply = input.into_reply(self.id);
        match reply.body.payload {
            Payload::Broadcast { message } => {
                reply.body.payload = Payload::BroadcastOk;
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to broadcast")?;
                output.write_all(b"\n").context("write trailing newline")?;
                // reply
                //     .serialize(output)
                //     .context("serialize response to echo")?;
                self.id += 1;
            }
            Payload::Read => {}
            Payload::Topology { .. } => {}
            Payload::ReadOk { messages } | Payload::BroadcastOk | Payload::TopologyOk => {}
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _>(())
}
