use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
#[pyfunction]
fn fib_first() {
    println!("fib first rs package")
}

#[pymodule(name = "fib_rs")]
fn fib_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib_first, m)?)
}
