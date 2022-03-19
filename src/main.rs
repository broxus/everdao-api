use std::env;

use anyhow::Result;

use bridge_dao_indexer::server::*;

#[tokio::main(worker_threads = 8)]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match &*args[1] {
        "server" => start_server().await?,
        other => anyhow::bail!("Unknown arg - {:?}", other),
    }

    Ok(())
}
