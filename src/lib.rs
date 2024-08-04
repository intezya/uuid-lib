use pyo3::prelude::*;

mod _uuid1;
mod _uuid2;
mod _uuid3;
mod _uuid5;
mod _uuid6;
mod _uuid7;
mod _uuid8;


#[pyfunction]
fn uuid1() -> PyResult<String> {
    let uuid1 = _uuid1::generate();
    Ok(uuid1.to_string())
}

#[pyfunction]
fn uuid2() -> PyResult<String> {
    let uuid2 = _uuid2::generate();
    Ok(uuid2.to_string())
}

#[pyfunction]
fn uuid6() -> PyResult<String> {
    let uuid6 = _uuid6::generate();
    Ok(uuid6.to_string())
}

#[pyfunction]
fn uuid7() -> PyResult<String> {
    let uuid7 = _uuid7::generate();
    Ok(uuid7.to_string())
}

#[pyfunction]
fn uuid8() -> PyResult<String> {
    let uuid8 = _uuid8::generate();
    Ok(uuid8.to_string())
}



#[pymodule]
fn _uuid_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(uuid1, m)?)?;
    m.add_function(wrap_pyfunction!(uuid2, m)?)?;
    m.add_function(wrap_pyfunction!(uuid6, m)?)?;
    m.add_function(wrap_pyfunction!(uuid7, m)?)?;
    m.add_function(wrap_pyfunction!(uuid8, m)?)?;
    Ok(())
}