use pyo3::types::PyBytes;
use pyo3::prelude::*;
use rand::RngCore;
use uuid::{Context, Timestamp, Uuid};
use mac_address::get_mac_address;

mod _uuid2;



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
fn uuid1() -> PyResult<String> {
    let uuid = Uuid::new_v1(
        Timestamp::from_unix(&Context::new_random(), 0, 0), &get_node_id()
    );
    Ok(uuid.to_string())
}

#[pyfunction]
fn uuid2() -> PyResult<String> {
    let uuid = _uuid2::generate();
    Ok(uuid.to_string())
}

#[pyfunction]
fn uuid3() -> PyResult<String> {
    let uuid = Uuid::new_v3(
        &Uuid::new_v4(), &get_node_id()
    );
    Ok(uuid.to_string())
}

#[pyfunction]
fn uuid4() -> PyResult<String> {
    let uuid = Uuid::new_v4();
    Ok(uuid.to_string())
}

#[pyfunction]
fn uuid5() -> PyResult<String> {
    let uuid = Uuid::new_v5(
        &Uuid::new_v4(), &get_node_id()
    );
    Ok(uuid.to_string())
}

#[pyfunction]
fn uuid6() -> PyResult<String> {
    let uuid = Uuid::now_v6(
        &get_node_id()
    );
    Ok(uuid.to_string())

}#[pyfunction]
fn uuid7() -> PyResult<String> {
    let uuid = Uuid::now_v7();
    Ok(uuid.to_string())

}#[pyfunction]
fn uuid8(bytes: &Bound<'_, PyBytes>) -> PyResult<String> {
    let uuid = Uuid::new_v8(
        bytes.extract()?
    );
    Ok(uuid.to_string())
}





#[pymodule]
fn _uuid_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
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