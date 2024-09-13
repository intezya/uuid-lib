use pyo3::{
    exceptions::PyNotImplementedError, prelude::PyAnyMethods, pyclass, pymethods, types::PyBytes, Bound, PyResult,
};
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
        self.uuid.hyphenated().to_string()
    }

    fn __repr__(&self) -> String {
        format!("UUID object:  UUID(\"{}\")", self.uuid)
    }

    fn __len__(&self) -> usize {
        self.uuid.hyphenated().to_string().chars().count()
    }

    #[getter]
    fn bytes(&self) -> &[u8] {
        self.uuid.as_bytes()
    }

    fn to_timestamp(&self) -> PyResult<u64> {
        match self.uuid.get_timestamp() {
            Some(ts) => {
                let (seconds, subsec_nanos) = ts.to_unix();
                Ok(seconds * 1000 + subsec_nanos as u64 / 1_000_000)
            }
            _ => Err(PyNotImplementedError::new_err("Timestamp not available for this uuid version!"))
        }
    }

    #[staticmethod]
    fn new_from_bytes(bytes: &Bound<'_, PyBytes>) -> PyResult<UUID> {
        let bytes: Bytes = bytes.extract()?;
        Ok(UUID {
            uuid: Uuid::from_bytes(bytes),
        })
    }
}