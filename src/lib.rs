use mac_address::get_mac_address;
use pyo3::prelude::{
    pyfunction, pymodule, wrap_pyfunction, Bound, PyAnyMethods, PyModule, PyModuleMethods, PyResult,
};
use pyo3::types::PyBytes;
use rand::RngCore;
use uuid::{Uuid};
use uuid_struct::UUID;

mod _uuid2;
mod uuid_struct;


fn get_node_id() -> [u8; 6] {
    let bytes = match get_mac_address() {
        Ok(Some(mac_address)) => mac_address.bytes(),
        _ => {
            let mut bytes = [0u8; 6];
            rand::thread_rng().fill_bytes(&mut bytes);
            bytes[0] = bytes[0] | 0x01;
            bytes
        }
    };
    bytes
}

#[pyfunction]
fn uuid1() -> PyResult<UUID> {
    let uuid = Uuid::now_v1(
        &get_node_id(),
    );
    Ok(UUID { uuid })
}

#[pyfunction]
fn uuid2() -> PyResult<UUID> {
    let uuid = _uuid2::now_v2();
    Ok(UUID { uuid })
}

#[pyfunction]
fn uuid3() -> PyResult<UUID> {
    let uuid = Uuid::new_v3(
        &Uuid::new_v4(),
        &get_node_id(),
    );
    Ok(UUID { uuid })
}

#[pyfunction]
fn uuid4() -> PyResult<UUID> {
    let uuid = Uuid::new_v4();
    Ok(UUID { uuid })
}

#[pyfunction]
fn uuid5() -> PyResult<UUID> {
    let uuid = Uuid::new_v5(
        &Uuid::new_v4(),
        &get_node_id(),
    );
    Ok(UUID { uuid })
}

#[pyfunction]
fn uuid6() -> PyResult<UUID> {
    let uuid = Uuid::now_v6(
        &get_node_id()
    );
    Ok(UUID { uuid })
}

#[pyfunction]
fn uuid7() -> PyResult<UUID> {
    let uuid = Uuid::now_v7();
    Ok(UUID { uuid })
}

#[pyfunction]
fn uuid8(bytes: &Bound<'_, PyBytes>) -> PyResult<UUID> {
    let uuid = Uuid::new_v8(
        bytes.extract()?
    );
    Ok(UUID { uuid })
}


#[pymodule]
fn _uuid_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<UUID>()?;
    m.add_function(wrap_pyfunction!(uuid1, m)?)?;
    m.add_function(wrap_pyfunction!(uuid2, m)?)?;
    m.add_function(wrap_pyfunction!(uuid3, m)?)?;
    m.add_function(wrap_pyfunction!(uuid4, m)?)?;
    m.add_function(wrap_pyfunction!(uuid5, m)?)?;
    m.add_function(wrap_pyfunction!(uuid6, m)?)?;
    m.add_function(wrap_pyfunction!(uuid7, m)?)?;
    m.add_function(wrap_pyfunction!(uuid8, m)?)?;
    Ok(())
}