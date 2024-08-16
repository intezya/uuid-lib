use pyo3::{prelude::PyAnyMethods, pyclass, pymethods, types::PyBytes, Bound, PyResult};
use uuid::{Bytes, Uuid};


#[pyclass(subclass, module = "uuid_lib")]
#[derive(Clone)]
pub struct UUID {
    pub uuid: Uuid,
}


#[pymethods]
impl UUID {
    #[new]
    fn init(
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
        return format!("UUID object:  UUID(\"{}\")", self.uuid);
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