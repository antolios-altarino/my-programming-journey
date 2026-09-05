use pyo3::prelude::*;

#[pyfunction]
fn calc(a: i32, b: i32, operator: &str) -> i32 {
    match operator {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => 0,
    }
}

#[pymodule]
fn calculator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc, m)?)?;
    Ok(())
}