use crate::api::requests::SearchTransfersRequest;
use crate::models::sqlx::TransferFromDb;
use crate::services::Services;

impl Services {
    pub async fn search_transfers(
        &self,
        input: SearchTransfersRequest,
    ) -> Result<(Vec<TransferFromDb>, i64), anyhow::Error> {
        self.sqlx_client.search_transfers(input).await
    }
}
