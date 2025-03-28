use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rasn(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    let submodule = PyModule::new(py, "types")?;
    // submodule.add_class::<::rasn::types::Any>()?;
    // submodule.add_class::<::rasn::types::InstanceOf>()?;
    // submodule.add_class::<::rasn::types::SequenceOf>()?;
    // submodule.add_class::<::rasn::types::SetOf>()?;
    submodule.add_class::<::rasn::types::ObjectIdentifier>()?;
    // submodule.add_class::<::rasn::types::Open>()?;
    // submodule.add_class::<::rasn::types::BitString>()?;
    submodule.add_class::<::rasn::types::Integer>()?;
    // submodule.add_class::<::rasn::types::BitString>()?;
    // submodule.add_class::<::rasn::types::BmpString>()?;
    // submodule.add_class::<::rasn::types::FixedBitString>()?;
    // submodule.add_class::<::rasn::types::FixedOctetString>()?;
    // submodule.add_class::<::rasn::types::GeneralString>()?;
    // submodule.add_class::<::rasn::types::GraphicString>()?;
    // submodule.add_class::<::rasn::types::Ia5String>()?;
    // submodule.add_class::<::rasn::types::NumericString>()?;
    // submodule.add_class::<::rasn::types::OctetString>()?;
    // submodule.add_class::<::rasn::types::PrintableString>()?;
    // submodule.add_class::<::rasn::types::TeletexString>()?;
    // submodule.add_class::<::rasn::types::Utf8String>()?;
    // submodule.add_class::<::rasn::types::VisibleString>()?;
    m.add_submodule(&submodule)?;
    Ok(())
}

#[pyfunction]
pub fn sequence(py: Python) -> PyResult<&PyCFunction> {
    let f = move |args: &PyTuple, _: Option<&PyDict>| -> PyResult<Py<PyCFunction>> {
        Python::with_gil(|py| {
            // Get the `func` parameter
            let func: PyObject = args.get_item(0)?.into();
            // Get the function name...
            let f_name = func
                .getattr(py, "__name__")
                .unwrap()
                .extract::<String>(py)
                .unwrap();
            println!("generating impl for {f_name}");
            let g = move |args: &PyTuple, kwargs: Option<&PyDict>| {
                // Print when the function is called
                println!("Calling function: {}", f_name);
                Python::with_gil(|py| func.call(py, args, kwargs))
            };
            match PyCFunction::new_closure(py, None, None, g) {
                Ok(r) => Ok(r.into()),
                Err(e) => Err(e),
            }
        })
    };
    // Return the enclosed decorator
    PyCFunction::new_closure(py, None, None, f)
}

fn extract_identity(value: String) -> (u8, String) {
    let arr: Vec<&str> = value.split('_').collect();
    (arr[1].parse::<u8>().unwrap(), String::from(arr[0]))
}

/// A Python module implemented in Rust.
#[pymodule]
fn types(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<::rasn::types::Integer>()?;
    Ok(())
}
