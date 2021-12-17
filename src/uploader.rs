use crate::utils::{chunk_vector, get_s3_client, regex_filter};
use crate::{Upload, CHUNK_SIZE};

use anyhow::Result;
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::ByteStream;
use futures::future::join_all;
use tokio::task::spawn;
use walkdir::{DirEntry, WalkDir};

/// Upload to S3 bucket
///
/// # Arguments
/// * `t` - Upload struct with parsed args from user input
///
/// # Return Values
/// Nothing
pub async fn uploader(t: Upload) -> Result<()> {
    // recursively list the input directory
    let mut all_files = list_input_directory(t.input.as_str());

    // filter paths
    if t.filter.is_some() {
        println!(
            "Filtering using the regular expression: {}",
            &t.filter.as_ref().unwrap()
        );
        all_files = regex_filter(all_files, t.filter.unwrap().as_str());
    }

    let no_files = all_files.len();

    if no_files == 0 {
        panic!("Found 0 files to upload. If you've used a filter check it's correct")
    }

    println!("{} objects to upload..", no_files);

    // allocate CHUNK_SIZE keys to n chunks, where n * 300 is the number of keys in the entire bucket
    let file_chunks = chunk_vector(all_files, CHUNK_SIZE);

    // check if bucket exists
    let (client, shared_config) = get_s3_client().await?;
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets.unwrap();

    if !buckets
        .iter()
        .map(|bucket| bucket.name.as_ref().unwrap())
        .any(|x| x == &t.bucket)
    {
        println!(
            "Bucket {} doesn't exist, attempting to create it ",
            &t.bucket
        );

        // this config setting is nasty but required
        let constraint = BucketLocationConstraint::from(shared_config.region().unwrap().as_ref());
        let cfg = CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();

        client
            .create_bucket()
            .create_bucket_configuration(cfg)
            .bucket(&t.bucket)
            .send()
            .await?;
        println!("Created bucket: {} ", &t.bucket);
    }

    let uploader_futures: Vec<_> = file_chunks
        .iter()
        .map(|file_chunk| {
            let bucket_c = t.bucket.clone();
            let file_chunk_c = file_chunk.clone();
            spawn(async move {
                upload_objects(file_chunk_c, bucket_c).await.unwrap();
            })
        })
        .collect();

    println!("Uploading...");
    join_all(uploader_futures).await;

    Ok(())
}

/// Check file isn't hidden
///
/// # Arguments
/// * `entry` - directory entry
///
/// # Return Values
/// * `hidden` - if file is hidden or not
fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| entry.depth() == 0 || !s.starts_with('.'))
}


/// Recursively list all files (not hidden) in the input dir to upload
///
/// # Arguments
/// * `input_dir` - directory to lies all files in
///
/// # Return Values
/// * `all_files` - a vector of file names
#[allow(clippy::redundant_closure)]
fn list_input_directory(input_dir: &str) -> Vec<String> {
    WalkDir::new(input_dir)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .filter(|t| t.metadata().unwrap().is_file())
        .map(|x| x.path().display().to_string())
        .collect::<Vec<_>>()
}

/// Upload a file chunk to a bucket
///
/// # Arguments
/// * `file_chunk` - a vector of file names to upload
/// * `bucket` - name of the bucket to upload to
///
/// # Return Values
/// Nothing
async fn upload_objects(file_chunk: Vec<String>, bucket: String) -> Result<()> {
    let (client, _) = get_s3_client().await?;

    for file in file_chunk {
        client
            .put_object()
            .bucket(&bucket)
            .key(&file)
            .body(ByteStream::from_path(&file).await.unwrap())
            .send()
            .await?;
    }

    Ok(())
}
