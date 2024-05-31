use anyhow::Result;
use zchat::serve::*;


#[tokio::main]
async fn main() -> Result<()> {

    let server: Server = Server::new();

    server.run().await?;

    Ok(())
}
