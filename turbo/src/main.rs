#![warn(clippy::pedantic)]

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::time::Instant;
use turbolib::{uploader, downloader};

// number of keys per task
const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(
about = "turbocharged S3 downloads and uploads",
version = VERSION_NUMBER,
author = "benjamin ellis <benjaminjellis@protonmail.com>",
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
        SubCommand::Upload(u) => {
            uploader(u.bucket, u.input, u.filter).await?;
        }
        SubCommand::Download(d) => {
            downloader(d.bucket, d.output, d.filter).await?;
        }
    }

    println!("\n");
    println!("Time taken: {}s", s.elapsed().as_secs());

    Ok(())
}
