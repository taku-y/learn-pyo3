use anyhow::Result;
use pyo3::{
    types::{IntoPyDict, /*PyModule, PyTuple*/},
    /*PyObject,*/ Python, /*ToPyObject*/
};

fn main() -> Result<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = [("np", py.import("numpy")?)].into_py_dict(py);

    // Create numpy array of f64
    let array = py.eval("np.zeros((3, 2), dtype=np.float64)", None, Some(&locals))?;
    let dtype: String = array.getattr("dtype")?.getattr("name")?.extract()?;
    println!("{:?}", array);
    println!("{:?}", dtype);

    Ok(())
}
