use numpy::ndarray::{Array3, ArrayView3, Axis};
use numpy::{IntoPyArray, PyArray3, PyArray4, PyArrayDyn, PyReadonlyArray4};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn process<'py>(py: Python<'py>, x: PyReadonlyArray4<f64>) -> &'py PyArray3<f64> {
        let array = x.as_array();
        let y = array.axis_iter(Axis(0)).next().unwrap();
        // let y: ArrayView3<f64>;
        // for ar in array.axis_iter(Axis(0)) {
        //     &y = ar;
        //     break;
        // }
        y.to_owned().into_pyarray(py)
        // y.into_pyarray(py)
    }
    Ok(())
}

// mod rust_fn {
//     use numpy::*;

//     pub fn max_min(x: &ArrayViewD<'_, f64>) -> Array1<f64> {
//         if x.len() == 0 {
//             return arr1(&[]); // If the array has no elements, return empty array
//         }
//         let max_val = x
//             .iter()
//             .map(|a| OrderedFloat(*a))
//             .max()
//             .expect("Error calculating max value.")
//             .0;
//         let min_val = x
//             .iter()
//             .map(|a| OrderedFloat(*a))
//             .min()
//             .expect("Error calculating min value.")
//             .0;
//         let result_array = arr1(&[max_val, min_val]);
//         result_array
//     }
// }
