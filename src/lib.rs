use image::{imageops, RgbImage};
use numpy::ndarray::{Array3, Axis};
use numpy::{IntoPyArray, PyArray3, PyArray4};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn process<'py>(py: Python<'py>, x: &PyArray4<u8>) {
        let mut x = unsafe { x.as_array_mut() };
        // let mut y = x.axis_iter_mut(Axis(0)).next().unwrap();
        // &y * 1;
        // for img in array.axis_iter(Axis(0)) {
        //     assert!(img.is_standard_layout());
        //     let (height, width, z) = y.dim();
        //     let raw = img.to_owned().into_raw_vec();
        //     let mut img_buff = RgbImage::from_raw(width as u32, height as u32, raw)
        //         .expect("container should have the right size for the image dimensions");
        //     imageops::flip_vertical_in_place(&mut img_buff);
        //     let raw_img_buff = img_buff.into_vec();
        //     let g = Array3::from_shape_vec((height, width, z), raw_img_buff).unwrap();
        // }
        for mut y in x.axis_iter_mut(Axis(0)) {
            y.fill(1);
            // y.fill(1);
            // // let mut y = &array.axis_iter_mut(Axis(0)).next().unwrap().to_owned();
            // assert!(y.is_standard_layout());
            // let (height, width, z) = y.dim();
            // let raw = y.to_owned().into_raw_vec();
            // let mut img = RgbImage::from_raw(width as u32, height as u32, raw)
            //     .expect("container should have the right size for the image dimensions");
            // imageops::flip_vertical_in_place(&mut img);
            // let raw_img = img.into_vec();
            // y = Array3::from_shape_vec((height, width, z), raw_img)
            //     .unwrap()
            //     .view_mut();
        }

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
