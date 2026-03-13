use pyo3::prelude::*;

#[pyfunction]
fn magic_number() -> String {
    format!("The magic number is {}", include_str!("magic_number.txt"))
}

#[pymodule]
fn maturin_marimo(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(magic_number, module)?)?;
    Ok(())
}
