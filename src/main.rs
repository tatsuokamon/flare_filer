mod cloud_flare_filer;
mod db_executor;
mod engine;
mod file_saver;
mod shared_architect;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{Client, config::Region};

macro_rules! get_env {
    ($keyword:expr) => {{
        tracing::info!("config: {}", &$keyword);
        std::env::var(&$keyword).expect(&format!("faied while loading env var: {}", &$keyword))
    }};
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("failed to load .env");

    let end_point_url = get_env!("END_POINT_URL");
    let region_provider = RegionProviderChain::first_try(Region::new("auto"));

    let config = aws_config::from_env()
        .region(region_provider)
        .endpoint_url(&end_point_url)
        .load()
        .await;

    let client = Client::from_conf(
        aws_sdk_s3::config::Builder::from(&config)
            .force_path_style(true)
            .build(),
    );

    let body = ByteStream::from_static(b"hello from rust to AWS");
    let bucket_name = get_env!("BUCKET_NAME");
    let file_name = get_env!("FILE_NAME");
    client
        .put_object()
        .bucket(&bucket_name)
        .key(file_name)
        .body(body)
        .send()
        .await
        .expect("failed while sending file");

    println!("upload success")
}
