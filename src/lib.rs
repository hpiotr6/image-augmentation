use image::RgbImage;
use numpy::ndarray::{Array3, ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::{pymodule, types::PyModule, PyResult, Python};

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // example using immutable borrows producing a new array
    fn axpy(a: f64, x: ArrayViewD<'_, f64>, y: ArrayViewD<'_, f64>) -> ArrayD<f64> {
        a * &x + &y
    }

    // example using a mutable borrow to modify an array in-place
    fn mult(a: f64, mut x: ArrayViewMutD<'_, f64>) {
        x *= a;
    }

    // wrapper of `axpy`
    #[pyfn(m)]
    #[pyo3(name = "axpy")]
    fn axpy_py<'py>(
        py: Python<'py>,
        a: f64,
        x: PyReadonlyArrayDyn<f64>,
        y: PyReadonlyArrayDyn<f64>,
    ) -> &'py PyArrayDyn<f64> {
        let x = x.as_array();
        let y = y.as_array();
        let z = axpy(a, x, y);
        z.into_pyarray(py)
    }

    // wrapper of `mult`
    #[pyfn(m)]
    #[pyo3(name = "mult")]
    fn mult_py(_py: Python<'_>, a: f64, x: &PyArrayDyn<f64>) {
        let x = unsafe { x.as_array_mut() };
        mult(a, x);
    }

    Ok(())
}

// fn array_to_image(arr: Array3<u8>) -> RgbImage {
//     assert!(arr.is_standard_layout());

//     let (height, width, _) = arr.dim();
//     let raw = arr.into_raw_vec();

//     RgbImage::from_raw(width as u32, height as u32, raw)
//         .expect("container should have the right size for the image dimensions")
// }

// fn hello_wordl() {
//     println!("Hello World!");
// }