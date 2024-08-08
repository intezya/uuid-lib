use pyo3::{Bound, pyclass, pymethods, PyResult};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyBytes;
use uuid::Uuid;
use uuid::Bytes;


#[pyclass(subclass, module="uuid_lib")]
#[derive(Clone)]
pub struct UUID {
    pub uuid: Uuid
}


#[pymethods]
impl UUID {
    #[new]
    fn new(
        bytes: &Bound<'_, PyBytes>,
    ) -> PyResult<Self> {

        let uuid = match bytes {
            bytes => Self::new_from_bytes(bytes),
        }?;
        Ok(uuid)
    }


    fn __str__(&self) -> String {
        return self.uuid.hyphenated().to_string();
    }

    fn __repr__(&self) -> String {
        return format!("UUID(\"{}\")", self.uuid)
    }

    #[getter]
    fn bytes(&self) -> &[u8] {
        self.uuid.as_bytes()
    }

    #[staticmethod]
    fn new_from_bytes(bytes: &Bound<'_, PyBytes>) -> PyResult<UUID> {
        let bytes: Bytes = bytes.extract()?;
        Ok(UUID {
            uuid: Uuid::from_bytes(bytes),
        })
    }
}