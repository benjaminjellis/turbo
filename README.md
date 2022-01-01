# turbo ![CI](https://github.com/benjaminjellis/turbo/actions/workflows/cd.yml/badge.svg) ![MSRV](https://img.shields.io/badge/msrv-1.56.0-red) ![version](https://img.shields.io/badge/version-0.2.5-blue)

turbo is a CLI tool for downloading and uploading large datasets from and to AWS S3 quickly. 

turbo is useful for downloading and uploading large machine learning datasets (e.g. of 10s of thousands of images) 
between S3 and a local or virtual machine used for model development.

## 1 Using Turbo

### 1.1 Installation

#### 1.1.1 Installation from source 
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

#### 1.1.2 Installation from pre-built binaries 
Pre-compiled binaries for windows, linux and mac are available from
[here](https://github.com/benjaminjellis/turbo/releases) for each release.

### 1.2 Setup 
turbo requires AWS secrets to access private buckets and for the region to be set. turbo uses the
[AWS Rust SDK](https://github.com/awslabs/aws-sdk-rust) so the preferred method is via env variables

These are

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


### 1.3 Running turbo 

#### 1.3.1 Download an entire bucket
To download an entire bucket (e.g. ```my_bucket```) navigate to the directory where you'd like to save the download to 
and run 

```shell
turbo download --bucket my_bucket --output data
```

This will download the ```my_bucket``` bucket into a directory called ```data```


#### 1.3.2 Using regular expressions to filter downloads
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

#### 1.3.3 Upload an entire directory 
To upload an entire local directory ```my_local_dir``` run 

```shell
turbo upload --input my_local_dir --bucket my_bucket
```

#### 1.3.4 Upload parts of a directory 
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


## 2. 🔪 Sharp Bits 

- The underlying AWS Rust SDK is developer preview so there may be bugs
- turbo's API isn't yet stable so may be subject to change across versions
