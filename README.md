# turbo 
![CI](https://github.com/benjaminjellis/turbo/actions/workflows/cd.yml/badge.svg) 
![MSRV](https://img.shields.io/badge/msrv-1.57.0-red) 
![version](https://img.shields.io/badge/version-0.3.0-blue)

turbo is a tool for downloading and uploading large datasets from and to AWS S3 quickly. turbo is available as a rust
library, python library or as a CLI tool. 

turbo is useful for downloading and uploading large machine learning datasets (e.g. of 10s or 100s of thousands of images) 
between S3 and a local or virtual machine used for model development / training.

## 1 Using Turbo

### 1.2 General Setup
For all use cases turbo requires AWS secrets to access private buckets and for the region to be set. turbo uses the
[AWS Rust SDK](https://github.com/awslabs/aws-sdk-rust) so the usual methods for providing credentials (i.e. credentials file
or env variables) are supported. 

For env variables the following need to be set:

    - AWS_ACCESS_KEY_ID
    - AWS_SECRET_ACCESS_KEY
    - AWS_REGION

These can be set like

```shell
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
export AWS_REGION=...
```

or using a ```.env``` file that looks like
```
AWS_ACCESS_KEY_ID=...
AWS_SECRET_ACCESS_KEY=...
AWS_REGION=...
```

turbo uses [dotenv](https://crates.io/crates/dotenv) so there's no need to source your ```.env``` file if you choose to
use one

### 1.3 Installation 


####  turbo - CLI tool

##### From source
To install from source you'll need to install rustup by following the instructions [here](https://rustup.rs/).

Then clone this repo
```shell
git clone git@github.com:benjaminjellis/turbo.git
```

Navigate to the cloned repo and run
```shell
cargo build --release
```

This will create a binary called ```turbo``` (or ```turbo.exe``` on Windows) in the directory
```target/release```.

If you add this binary to a location in your path you'll be able to run turbo.


##### From pre-built binaries
Pre-compiled binaries for windows, linux and mac are available from
[here](https://github.com/benjaminjellis/turbo/releases) for each release.

#### turbolib - Rust Library
```turbolib``` is the back end for the CLI tool ```turbo```, ```turbolib``` is distributed via [crates.io](crates.io).

To use ```turbolib``` simply ass it to the dependencies section of your Cargo.toml

```Cargo.toml
[dependencies]
turbolib = "*"
```

#### py-turbo - Python package
py-turbo is a python package that serves as python bindings for ```turbolib```


### 1.4 Usage


#### turbo

##### Download a bucket 
To download an entire bucket (e.g. ```my_bucket```) navigate to the directory where you'd like to save the download to
and run

```shell
turbo download --bucket my_bucket --output data
```

This will download the ```my_bucket``` bucket into a directory called ```data```

##### Using regular expressions to filter downloads
Using the ```--filter``` flag, regular expressions can be used to specify what in a bucket to download.

For example take a bucket ```my_bucket``` that has three sub folders: ```test```, ```train``` and ```val```

```
📂 my_bucket
┣ 📂 test
┃ ┣ somefile.txt
┃ ┣ another_file.txt
┃ ┃ ...
┃ ┗ etc.
┃
┣ 📂 train
┃ ┣ somefile.txt
┃ ┣ another_file.txt
┃ ┃ ...
┃ ┗ etc.
┃ 
┣ 📂 val
┃ ┣ somefile.txt
┃ ┣ another_file.txt
┃ ┃ ...
┃ ┗ etc
```

To download just the ```val``` directory you can run

```shell
turbo download --bucket my_bucket --output data --filter 'val/*'
```

note the single quotes around ```val/*```

##### Uploading a directory
To upload an entire local directory ```my_local_dir``` run

```shell
turbo upload --input my_local_dir --bucket my_bucket
```

##### Uploading using filters
Using the ```--filter``` flag, regular expressions can be used to specify what in a bucket to upload.

For example take a bucket ```my_local_dir``` that has three subdirectories: ```test```, ```train``` and ```val```

```
📂 my_local_dir
┣ 📂 test
┃ ┣ somefile.txt
┃ ┣ another_file.txt
┃ ┃ ...
┃ ┗ etc.
┃
┣ 📂 train
┃ ┣ somefile.txt
┃ ┣ another_file.txt
┃ ┃ ...
┃ ┗ etc.
┃ 
┣ 📂 val
┃ ┣ somefile.txt
┃ ┣ another_file.txt
┃ ┃ ...
┃ ┗ etc
```

To upload just the ```val``` directory you can run

```shell
turbo upload --input my_local_dir --bucket my_bucket --filter 'val/*'
```

#### py-turbo

```py-turbo``` provides the same uploading and downloading via two functions:
- ```upload```
- ```download```

Note that because the backend ```turbolib``` is async so are the python functions 

```python
from py_turbo import download, upload
import asyncio


async def main():
    await download(bucket="my-bucket", output="./data")
    await download(bucket="my-other-bucket", input="")


if __name__ == "__main__":
    asyncio.run(main())
```

The same filtering that ```turbo``` allows can be used as in by specifying the regular expression using the ```filter``` 
kwarg. 

## 2. 🔪 Sharp Bits 

- The underlying AWS Rust SDK is developer preview so there may be bugs
- turbo's API isn't yet stable so may be subject to change across versions
