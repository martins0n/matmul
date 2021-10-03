use ndarray::Array2;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_npy(fname: &Path, matrix_shape: (usize, usize)) -> Array2<f64> {
    let mut file = File::open(fname).unwrap();

    let mut buffer: Vec<u8> = Vec::new();

    let _ = file.read_to_end(&mut buffer).unwrap();

    let tokens_to_skip = (((buffer[9] as u16) << 8) | buffer[8] as u16) as usize + 6 + 4;

    let mut array = Array2::<f64>::zeros(matrix_shape);

    for (idx, float_bytes) in buffer[tokens_to_skip..].chunks(8).enumerate() {
        let mut float64_: u64 = 0;
        for (idy, &byte_) in float_bytes.iter().enumerate() {
            float64_ = (byte_ as u64) << 8 * (idy) | float64_;
        }
        array[[idx / matrix_shape.1, idx % matrix_shape.1]] = f64::from_bits(float64_);
    }

    array
}
