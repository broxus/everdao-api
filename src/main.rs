use anyhow::Result;

use bridge_dao_indexer::server::*;

#[tokio::main(worker_threads = 8)]
async fn main() -> Result<()> {
    start_server().await
}
