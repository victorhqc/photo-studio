use crate::utils::encode_url_component;
use gotham::hyper::Client;
use hyper_tls::HttpsConnector;
use rusoto_core::{ByteStream, HttpClient, Region, RusotoError};
use rusoto_credential::EnvironmentProvider;
use rusoto_s3::{PutObjectError, PutObjectOutput, PutObjectRequest, S3Client, S3};
use snafu::{Backtrace, ResultExt};
use std::env;

pub async fn upload(
    key: String,
    content_type: Option<String>,
    data: Vec<u8>,
) -> Result<PutObjectOutput> {
    let byte_stream = ByteStream::from(data);
    let hyper_builder = Client::builder();
    let https_connector = HttpsConnector::new();
    let http_client = HttpClient::from_builder(hyper_builder, https_connector);

    let credentials_provider = EnvironmentProvider::default();
    let s3 = S3Client::new_with(http_client, credentials_provider, Region::default());

    let bucket = env::var("AWS_S3_BUCKET_NAME").context(NoBucket)?;

    let input = PutObjectRequest {
        key,
        body: Some(byte_stream),
        bucket,
        acl: None,
        cache_control: None,
        content_disposition: None,
        content_encoding: None,
        content_language: None,
        content_length: None,
        content_md5: None,
        content_type,
        expires: None,
        grant_full_control: None,
        grant_read: None,
        grant_read_acp: None,
        grant_write_acp: None,
        metadata: None,
        object_lock_legal_hold_status: None,
        object_lock_mode: None,
        object_lock_retain_until_date: None,
        request_payer: None,
        sse_customer_algorithm: None,
        sse_customer_key: None,
        sse_customer_key_md5: None,
        ssekms_encryption_context: None,
        ssekms_key_id: None,
        server_side_encryption: None,
        storage_class: None,
        tagging: None,
        website_redirect_location: None,
    };

    let s3_object = s3.put_object(input).await.context(S3Issue)?;

    Ok(s3_object)
}

pub fn get_url(key: String) -> Result<String> {
    let no_spaces = key.replace(" ", "");
    let encoded = encode_url_component(no_spaces);

    let bucket = env::var("AWS_S3_BUCKET_NAME").context(NoBucket)?;

    Ok(format!(
        "https://{}.s3.{}.amazonaws.com/{}",
        bucket,
        Region::default().name(),
        encoded
    ))
}

pub type Result<T> = std::result::Result<T, AwsS3Error>;

#[derive(Debug, Snafu)]
pub enum AwsS3Error {
    #[snafu(display("Bucket name is not defined: {}", source))]
    NoBucket {
        source: std::env::VarError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not upload file to S3: {}", source))]
    S3Issue {
        source: RusotoError<PutObjectError>,
        backtrace: Backtrace,
    },
}
