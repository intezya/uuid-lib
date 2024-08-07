use pyo3::{pyclass, pymethods};
use uuid::Uuid;

#[pyclass(subclass, module="uuid_lib")]
pub struct UUID {
    pub uuid: Uuid
}


#[pymethods]
impl UUID {
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

}