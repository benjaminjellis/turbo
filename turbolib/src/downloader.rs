use crate::utils::{chunk_vector, get_s3_client, regex_filter};
use crate::{CHUNK_SIZE, TurbolibError};

use aws_sdk_s3::Client;
use futures::future::join_all;
use std::fs::{create_dir_all, write};
use std::path::Path;
use tokio::task::spawn;


/// Download from S3 bucket
///
/// # Arguments
/// * `bucket` - the name of the bucket to download from
/// * `output` - the name of the local directory to download to
/// * `filter` - the regex filter used to pick objects to downloads
///
/// # Errors
/// Throws ```TurbolibError```
///
/// # Return Values
/// Nothing
pub async fn downloader(bucket: String, output: String, filter: Option<String>) -> Result<(), TurbolibError> {
    let (client, _) = get_s3_client().await?;

    create_dir_all(output.as_str())?;

    let mut all_keys = list_all_objects_in_bucket(client, &bucket).await?;

    // if a filter is provided use it
    if let Some(f) = filter{
        println!(
            "Filtering using the regular expression: {}",
            f.as_str()
        );
        all_keys = regex_filter(all_keys, f.as_str());
    }

    let no_keys = all_keys.len();

    if no_keys == 0usize{
        return Err(TurbolibError::NoObjectsFoundInBucket(bucket));
    }

    println!("{} objects to download..", no_keys);

    // allocate CHUNK_SIZE keys to n chunks, where n * 300 is the number of keys in the entire bucket
    let key_chunks = chunk_vector(all_keys, CHUNK_SIZE);

    // then spawn a new task for each of the n chunks to download the keys
    let downloader_futures: Vec<_> = key_chunks
        .iter()
        .map(|key_chunk| {
            let bucket_c = bucket.clone();
            let output_dir = output.clone();
            let key_chunk_c = key_chunk.clone();
            spawn(async move {
                download_objects(output_dir, bucket_c, key_chunk_c)
                    .await
                    .unwrap_err();
            })
        })
        .collect();
    println!("\n");

    println!("Downloading...");
    join_all(downloader_futures).await;

    Ok(())
}

/// List everything in a bucket
///
/// This function requires a pre-spawned client, it's not currently possible to list a bucket using
/// multiple clients and at present it's not a bottleneck
///
/// # Arguments
/// * `client` - s3 client used to communicate with aws
///
/// * `bucket` - name of the bucket to list contents of
///
/// # Return Values
///
/// * `objects` - vector of all the object keys in the bucket, at present no metadata is collected
///
pub async fn list_all_objects_in_bucket(client: Client, bucket: &str) -> Result<Vec<String>, TurbolibError> {
    // vec to store all the objects - this is dynamic, probably can't use capacity
    let mut objects: Vec<String> = vec![];

    let resp = client.list_objects_v2().bucket(bucket).send().await?;

    for object in resp.contents().unwrap_or_default() {
        objects.push(object.key().unwrap_or_default().to_owned());
    }

    let mut continuation_token = resp.next_continuation_token;

    // loop through all possible continuation tokens
    while continuation_token.is_some() {
        let resp = client
            .list_objects_v2()
            .bucket(bucket)
            .continuation_token(continuation_token.as_ref().unwrap())
            .send()
            .await?;

        for object in resp.contents().unwrap_or_default() {
            objects.push(object.key().unwrap_or_default().to_owned());
        }

        continuation_token = resp.next_continuation_token;
    }

    Ok(objects)
}

/// Download a vector of S3 keys
///
/// This function will spawn it's own client to download all of the keys in the ```keys``` parameters
///
/// # Arguments
/// * `output_dir` - output directory to save files to
///
/// * `bucket` - the s3 bucket to download from
///
/// * `region` - the region of the s3 bucket
///
/// * `keys` - a vector of the keys to download
///
///
/// # Return Values
/// Nothing
pub async fn download_objects(output_dir: String, bucket: String, keys: Vec<String>) -> Result<(), TurbolibError> {
    let (client, _) = get_s3_client().await?;
    for key in keys {
        let dl_resp = client.get_object().bucket(&bucket).key(&key).send().await?;

        let object = dl_resp.body.collect().await?.into_bytes();

        let m = format!("{}/{}", &output_dir, key);

        let output_path = Path::new(m.as_str());

        create_dir_all(&output_path.parent().unwrap())?;
        if !object.is_empty() {
            write(output_path, object)?;
        }
    }

    Ok(())
}
