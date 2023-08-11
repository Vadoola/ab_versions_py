use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
extern crate ab_versions as abv;

#[pyclass]
pub struct FileVersion(abv::FileVersion);

impl From<abv::FileVersion> for FileVersion {
    fn from(other: abv::FileVersion) -> Self {
        FileVersion(other)
    }
}

#[pymethods]
impl FileVersion {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

#[pyfunction]
pub fn get_version(filename: &str) -> PyResult<FileVersion> {
    match abv::get_version(&filename) {
        Ok(fv) => Ok(fv.into()),
        Err(e) => Err(PyValueError::new_err(e.to_string()))
    }
}

#[pyfunction]
pub fn is_protected(path: &str) -> PyResult<bool> {
    abv::is_protected(&path).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
pub fn strip_protection(path: &str) -> PyResult<()> {
    abv::strip_protection(path).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pymodule]
fn ab_versions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FileVersion>()?;
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_function(wrap_pyfunction!(is_protected, m)?)?;
    m.add_function(wrap_pyfunction!(strip_protection, m)?)?;

    Ok(())
}