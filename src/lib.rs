use pyo3::prelude::*;
use thread_priority::ThreadPriority;
use std::convert::TryInto;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn set_current_thread_priority(value: usize) -> PyResult<String> {
    // The lower the number the lower the priority.
    let val = value as u8;
    println!("{}", &val);
    let tp = ThreadPriority::Crossplatform(val.try_into().unwrap());
    println!("{:?}", &tp);
    ThreadPriority::Crossplatform(val.try_into().unwrap()).set_for_current().unwrap();
    Ok("".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pythreadpriority(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(set_current_thread_priority, m)?)?;
    Ok(())
}
