mod downloader;
mod uploader;
mod utils;

use downloader::downloader;
use uploader::uploader;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::time::Instant;

// number of keys per task
const CHUNK_SIZE: usize = 300;

#[derive(Parser)]
#[clap(
    version = "v0.2.3",
    author = "benjamin ellis <benjaminjellis@protonmail.com>"
)]
struct Opts {
    /// sub commands
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Download(Download),
    Upload(Upload),
}

/// Download from S3
#[derive(Parser)]
pub struct Download {
    /// Name of the S3 bucket to download
    #[clap(short, long)]
    pub bucket: String,
    /// Output directory to download to
    #[clap(short, long)]
    pub output: String,
    /// Optional: Filter what keys to download using regular expressions
    #[clap(short, long)]
    pub filter: Option<String>,
}

/// Upload to S3
#[derive(Parser)]
pub struct Upload {
    /// Name of the S3 to upload to
    #[clap(short, long)]
    pub bucket: String,
    /// Local directory of files to upload
    #[clap(short, long)]
    pub input: String,
    /// Optional: Filter what keys to upload using regular expressions
    #[clap(short, long)]
    pub filter: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let s = Instant::now();

    let opts: Opts = Opts::parse();

    // used to mash .env files with env variables
    dotenv().ok();

    match opts.subcmd {
        SubCommand::Upload(t) => {
            uploader(t).await?;
        }
        SubCommand::Download(t) => {
            downloader(t).await?;
        }
    }

    println!("\n");
    println!("Time taken: {}s", s.elapsed().as_secs());

    Ok(())
}
