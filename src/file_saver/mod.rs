use std::fmt::Debug;

use axum::body::Bytes;

pub trait FileSaver: Send + Sync + 'static {
    type Err: Debug;
    type SendOutput;
    type Body: From<Bytes>;

    async fn send_file(
        &self,
        file_name: &str,
        body: Self::Body,
    ) -> Result<Self::SendOutput, Self::Err>;
}
