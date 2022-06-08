use numpy::PyArray4;
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn process<'py>(_py: Python<'py>, x: &PyArray4<u8>) {
        let mut x = unsafe { x.as_array_mut() };
        rust_fn::do_sth(&mut x);
    }
    Ok(())
}

mod rust_fn {
    use image::{imageops, RgbImage};
    use numpy::ndarray::ArrayViewMut4;
    use numpy::ndarray::{Array3, Axis};

    pub fn do_sth(x: &mut ArrayViewMut4<'_, u8>) {
        for mut y in x.axis_iter_mut(Axis(0)) {
            assert!(y.is_standard_layout());
            let (height, width, z) = y.dim();
            let raw = y.to_owned().into_raw_vec();
            let mut img = RgbImage::from_raw(width as u32, height as u32, raw)
                .expect("container should have the right size for the image dimensions");
            imageops::flip_vertical_in_place(&mut img);
            let raw_img = img.into_vec();
            Array3::from_shape_vec((height, width, z), raw_img)
                .unwrap()
                .move_into(&mut y);
        }
    }
}
