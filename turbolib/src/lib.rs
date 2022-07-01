#![warn(clippy::pedantic)]

mod downloader;
mod uploader;
mod utils;

pub use downloader::downloader;
pub use uploader::uploader;

const CHUNK_SIZE: usize = 300;
