use aws_sdk_s3::{
    Client, Config,
    error::SdkError,
    operation::put_object::{PutObjectError, PutObjectOutput},
    primitives::ByteStream,
};

use crate::{file_saver::FileSaver, shared_architect::BuilderTrait};

#[derive(thiserror::Error, Debug)]
pub enum CloudFlareFilerErr {
    #[error("CloudFlareErr: FromSdk: {0}")]
    FromSdk(#[from] SdkError<PutObjectError>),
}

pub struct CloudFlareFiler {
    client: aws_sdk_s3::Client,
    bucket_name: String,
}

pub struct CloudFlareFilerBuilder {
    config: Config,
    buckt_name: String,
}

impl BuilderTrait for CloudFlareFilerBuilder {
    type Target = CloudFlareFiler;
    type Err = ();

    fn build(self) -> impl Future<Output = Result<Self::Target, Self::Err>> {
        async {
            let client = Client::from_conf(self.config);

            Ok(CloudFlareFiler {
                client,
                bucket_name: self.buckt_name,
            })
        }
    }
}

impl FileSaver for CloudFlareFiler {
    type Err = CloudFlareFilerErr;
    type SendOutput = PutObjectOutput;
    type Body = ByteStream;

    async fn send_file(
        &self,
        file_name: &str,
        body: Self::Body,
    ) -> Result<Self::SendOutput, Self::Err> {
        Ok(self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .body(body)
            .send()
            .await?)
    }
}
