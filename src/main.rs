use anyhow::Result;

use dao_api::server::*;

#[tokio::main(worker_threads = 8)]
async fn main() -> Result<()> {
    start_server().await
}
