use pyo3::{prelude::*, wrap_pyfunction};
use turbolib::{downloader, uploader};
use dotenv::dotenv;

/// download(bucket, output, filter, /)
/// --
///
/// Download from S3
#[pyfunction]
fn download(py: Python, bucket: String, output: String, filter: Option<String>) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        dotenv().ok();
        downloader(bucket, output, filter).await.unwrap();
        Ok(())
    })
}



/// upload(bucket, input, filter, /)
/// --
///
/// Upload to S3
#[pyfunction]
fn upload(py: Python, bucket: String, input: String, filter: Option<String>) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        dotenv().ok();
        uploader(bucket, input, filter).await.unwrap();
        Ok(())
    })
}

#[pymodule]
fn py_turbo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(download, m)?)?;
    m.add_function(wrap_pyfunction!(upload, m)?)?;
    Ok(())
}