use ndarray::Array2;

pub fn loop_interchange(
    a: &Array2<f64>,
    b: &Array2<f64>,
    c: &mut Array2<f64>,
    a_shape: (usize, usize),
    b_shape: (usize, usize),
) {
    for i in 0..a_shape.0 {
        for k in 0..a_shape.1 {
            for j in 0..b_shape.1 {
                c[[i, j]] += a[[i, k]] * b[[k, j]]
            }
        }
    }
}
