use aws_sdk_s3::primitives::ByteStream;

use crate::file_saver::FileSaver;

pub struct CloudFlareFiler {}

impl FileSaver for CloudFlareFiler {
    type Err = ();
    type SendOutput = ();
    type Body = ByteStream;

    async fn send_file(
        &self,
        file_name: &str,
        body: Self::Body,
    ) -> Result<Self::SendOutput, Self::Err> {
        Ok(())
    }
}
