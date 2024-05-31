use anyhow::Result;
use futures::{stream::SplitStream, SinkExt, StreamExt};
use std::{fmt, net::SocketAddr, sync::Arc};
use dashmap::DashMap;
use tokio::{
    net::TcpStream,
    sync::mpsc,
};
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{info, warn};

const MAX_EVENTS: usize = 128;

#[derive(Debug, Default)]
pub struct State {
    peers: DashMap<SocketAddr, mpsc::Sender<Arc<Event>>>,
}

struct Peer {
    username: String,
    stream: SplitStream<Framed<TcpStream, LinesCodec>>,
}

enum Event {
    Joined(String),
    Left(String),
    Chat{ sender: String, content: String},
}

pub async fn handle_client(
    state: Arc<State>, 
    addr: SocketAddr, 
    stream: TcpStream,
) -> Result<(), anyhow::Error> {
    // process the steam data
    let mut framed = Framed::new(stream, LinesCodec::new());

    // who connected on
    framed.send("Enter your name:").await?;
    let username = match framed.next().await {
        Some(Ok(username)) => username,
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(()),
    };

    // join and tell others
    let mut peer = state.add(addr, username, framed).await;
    let event = Arc::new(Event::user_joined(&peer.username));
    info!("{}", event);
    state.broadcast(addr, event).await;

    // communicate
    while let Some(line) = peer.stream.next().await {
        let rline = match line {
            Ok(_line) => _line,
            Err(e) => {
                warn!("Failed to read line from {}: {}", addr, e);
                break;
            }
        };

        let msg = Arc::new(Event::chat(&peer.username, rline));
        state.broadcast(addr, msg).await;
    }
    // left from here
    state.peers.remove(&addr);
    let msg = Arc::new(Event::user_left(&peer.username));
    state.broadcast(addr, msg).await;

    Ok(())
}

impl State {
    async fn add(
        &self,
        addr: SocketAddr,
        username: String,
        stm: Framed<TcpStream, LinesCodec>,
    ) -> Peer {
        let (tx, mut rx) = mpsc::channel(MAX_EVENTS);
        self.peers.insert(addr, tx);

        let (mut stream_sender, stream_receiver) = stm.split();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = stream_sender.send(msg.to_string()).await {
                    warn!("Failed to send msg to {}: {}", addr, e);
                    break;
                }
            }
        });

        Peer {
            username,
            stream: stream_receiver,
        }
    }

    async fn broadcast(&self, addr: SocketAddr, event: Arc<Event>) {
        for peer in self.peers.iter() {
            if peer.key() == &addr {
                continue;
            }
            if let Err(e) = peer.value().send(event.clone()).await {
                warn!("Failed to send message to {}: {}", peer.key(), e);
                self.peers.remove(peer.key());
            }
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Joined(content) => write!(f, "[{}]", content),
            Self::Left(content) => write!(f, "[{} :(]", content),
            Self::Chat { sender, content } => write!(f, "{}: {}", sender, content),
        }
    }
}

impl Event {
    fn user_joined(name: &str) -> Self {
        let content = format!("{} has joined the chat", name);
        Self::Joined(content)
    }

    fn user_left(name: &str) -> Self {
        let content = format!("{} has left the chat", name);
        Self::Left(content)
    }

    fn chat(sender: impl Into<String>, content: impl Into<String>) -> Self {
        Self::Chat {
            sender: sender.into(),
            content: content.into(),
        }
    }
}
