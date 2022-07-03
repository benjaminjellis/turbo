#![warn(clippy::pedantic)]

mod downloader;
mod uploader;
mod utils;

pub use downloader::downloader;
pub use uploader::uploader;

use thiserror::Error;

const CHUNK_SIZE: usize = 300;

#[derive(Error, Debug)]
pub enum TurbolibError {
    #[error("No objects found in the bucket: '{0}'. Check the bucket and any filters")]
    NoObjectsFoundInBucket(String),
    #[error("No files found in the directory: '{0}'. Check the directory and any filters")]
    NoFilesFoundInDirectory(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    ListBucketsError(#[from] aws_sdk_s3::types::SdkError<aws_sdk_s3::error::ListBucketsError>),
    #[error(transparent)]
    CreateBucketsError(#[from] aws_sdk_s3::types::SdkError<aws_sdk_s3::error::CreateBucketError>),
    #[error(transparent)]
    PutObjectError(#[from] aws_sdk_s3::types::SdkError<aws_sdk_s3::error::PutObjectError>),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    GetObjectError(#[from] aws_sdk_s3::types::SdkError<aws_sdk_s3::error::GetObjectError>),
    #[error(transparent)]
    ByteStreamError(#[from] aws_smithy_http::byte_stream::Error),
    #[error(transparent)]
    ListObjectsError(#[from] aws_sdk_s3::types::SdkError<aws_sdk_s3::error::ListObjectsV2Error>),


}