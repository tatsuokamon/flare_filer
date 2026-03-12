mod db_executor;

use std::fmt::Debug;

use axum::body::Bytes;
pub use db_executor::DBExecutor;

pub trait DBExecutorTrait: Send + Sync + 'static {
    type Err: Debug;
    type FileInfo;

    async fn ready_file_info(
        &self,
        file_name: &str,
        bytes: &Bytes,
    ) -> Result<Self::FileInfo, Self::Err>;
    async fn save_file_info(&self, file_info: &Self::FileInfo) -> Result<(), Self::Err>;
}
