# turbolib ![CI](https://github.com/benjaminjellis/turbo/actions/workflows/turbolib-publish.yml/badge.svg)

turbolib is a library for uploading and downloading many files to AWS S3 quickly. It's used as 
a backend for a cli tool called turbo and for a python library called [turbos3-py](https://pypi.org/project/turbos3-py/)

# Usage 

turbolib exposes two functions
- downloader
- uploader 

These are used to download and upload. 

E.g. to download an entire bucket (e.g. ```my_bucket```) into a local directory called ```data```

```rust 
use turbolib::{uploader, downloader};


downloader(my_bucket.into(), "data'.into(), None).await?;
```

you can also use regex filters, i.e. to download files that start end with ".txt"


```rust 
use turbolib::{uploader, downloader};


downloader(my_bucket.into(), "data'.into(), Some("*.txt).into())).await?;
```

