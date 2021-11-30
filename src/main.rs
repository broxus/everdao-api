use std::env;

use bridge_dao_indexer::server::*;
use dexpa::errors::*;

#[tokio::main(worker_threads = 8)]
async fn main() -> StdResult<()> {
    let args: Vec<String> = env::args().collect();

    match &*args[1] {
        "server" => start_server().await?,
        other => return Err(format!("Unknown arg - {}", other).into()),
    }

    Ok(())
}
