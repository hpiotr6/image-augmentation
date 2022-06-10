use numpy::PyArray4;
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn process_parallel<'py>(py: Python<'py>, x: &PyArray4<u8>) {
        let mut x = unsafe { x.as_array_mut() };
        py.allow_threads(|| rust_fn::do_flip(&mut x));
    }
    #[pyfn(m)]
    fn process_sequential<'py>(_py: Python<'py>, x: &PyArray4<u8>) {
        let mut x = unsafe { x.as_array_mut() };
        rust_fn::do_flip_seq(&mut x);
    }
    Ok(())
}

mod rust_fn {
    extern crate ndarray;
    use image::{imageops, ImageBuffer, Rgb, RgbImage};
    use ndarray::parallel::prelude::*;
    use numpy::ndarray::{Array3, ArrayViewMut4, Axis};

    pub fn do_flip(x: &mut ArrayViewMut4<'_, u8>) {
        x.axis_iter_mut(Axis(0)).into_par_iter().for_each(|mut y| {
            assert!(y.is_standard_layout());
            let (height, width, z) = y.dim();
            let raw = y.to_owned().into_raw_vec();
            let mut img = RgbImage::from_raw(width as u32, height as u32, raw)
                .expect("container should have the right size for the image dimensions");
            flip(&mut img);
            let raw_img = img.into_vec();
            Array3::from_shape_vec((height, width, z), raw_img)
                .unwrap()
                .move_into(&mut y);
        });
    }
    pub fn do_flip_seq(x: &mut ArrayViewMut4<'_, u8>) {
        x.axis_iter_mut(Axis(0)).for_each(|mut y| {
            assert!(y.is_standard_layout());
            let (height, width, z) = y.dim();
            let raw = y.to_owned().into_raw_vec();
            let mut img = RgbImage::from_raw(width as u32, height as u32, raw)
                .expect("container should have the right size for the image dimensions");
            flip(&mut img);
            let raw_img = img.into_vec();
            Array3::from_shape_vec((height, width, z), raw_img)
                .unwrap()
                .move_into(&mut y);
        });
    }

    fn flip(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        imageops::flip_vertical_in_place(img);
    }
}
