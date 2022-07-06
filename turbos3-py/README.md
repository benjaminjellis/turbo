# turbos3-py 
```turbos3-py``` is a python package for turbocharged S3 downloads and uploads.

## Install 

```turbos3-py``` is available via pip

```shell
pip install turbos3-py
```

## Setup
```turbos3-py``` uses the same auth methods as boto3 and the AWS CLI i.e. credentials file and env variables. See [here](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-quickstart.html)
for more details

## Usage 

```python
from turbos3_py import download, upload
import asyncio


async def main():
    await download(bucket="my-bucket", output="./data")
    await upload(bucket="my-other-bucket", input="./some_local_dir")


if __name__ == "__main__":
    asyncio.run(main())
```