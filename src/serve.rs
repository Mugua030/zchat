use crate::handler::*;
use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::warn;

#[async_trait::async_trait]
pub trait Serve {
    fn init(self);
    async fn run(self) -> Result<(), anyhow::Error>;
    fn stop(&self);
}

// process the request
pub struct Server {}

enum ServeError {
    InvalidArgError(String),
    NetError(String),
}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
}

#[async_trait::async_trait]
impl Serve for Server {
    fn init(self) {}

    async fn run(self) -> Result<()> {
        let addr = "0.0.0.0:8080";
        let listener = TcpListener::bind(addr).await?;
        let state = Arc::new(State::default());

        loop {
            let (stream, addr) = listener.accept().await?;
            let state_cloned = state.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_client(state_cloned, addr, stream).await {
                    warn!("Failed to handle client {}: {}", addr, e);
                }
            });
        }
    }

    fn stop(&self) {}
}
