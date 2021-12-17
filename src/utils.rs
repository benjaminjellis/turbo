use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use aws_types::config::Config;
use regex::Regex;

/// Function to filter the keys of an entire bucket
///
/// Filtering is done using regex from user input
///
/// # Arguments
/// * `keys` - vector of the keys in the bucket
/// * `regex_expression` - the regular expression to use to filter the keys
///
/// # Return Values
/// filtered keys
pub fn regex_filter(mut keys: Vec<String>, regex_expression: &str) -> Vec<String> {
    let re = Regex::new(regex_expression).unwrap();
    keys.retain(|x| re.is_match(x));
    keys
}

/// Turn (chunk) a vector of keys into a vector of vectors of keys
///
/// # Arguments
/// * `keys` - vectors of keys
/// * `chunk_size` - size of each vector in the output vector
///
/// # Return Values
/// * `key_chunks`
pub fn chunk_vector<T>(keys: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut keys_peekable = keys.into_iter().peekable();

    let mut key_chunks = vec![];

    while keys_peekable.peek().is_some() {
        let chunk: Vec<T> = keys_peekable.by_ref().take(chunk_size).collect();
        key_chunks.push(chunk);
    }

    key_chunks
}

/// Get an S3 client
///
/// # Arguments
/// None
///
/// # Return Values
/// * `client` - S3 client
pub async fn get_s3_client() -> Result<(Client, Config)> {
    let region_provider = RegionProviderChain::default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    // client to communicate with S3
    Ok((Client::new(&shared_config), shared_config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_filter() {
        // this is only a very simple regex test
        let test_keys = vec![
            String::from("test/1234.jpg"),
            String::from("train/1234.jpg"),
        ];
        let m = regex_filter(test_keys, "test/*");
        assert_eq!(m, vec![String::from("test/1234.jpg")]);
    }

    #[test]
    fn test_chunk_vector() {
        let int_vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let vector_chunks = chunk_vector(int_vector, 1);
        let mut counter = 1usize;
        for entry in vector_chunks {
            assert_eq!(entry[0], counter);
            counter += 1;
        }
    }

    #[tokio::test]
    async fn test_get_s3_client() {
        let (_client, _shared_config) = get_s3_client().await.unwrap();
    }
}
