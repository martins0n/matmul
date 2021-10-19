use ndarray::Array2;
use ndarray::ArrayView;

/// Matmul using iterators
/// asm:
/// .LBB130_49:
///      vmulpd  ymm1, ymm0, ymmword, ptr, [r8, +, 8*rdx, -, 96]
///      vmulpd  ymm2, ymm0, ymmword, ptr, [r8, +, 8*rdx, -, 64]
///      vmulpd  ymm3, ymm0, ymmword, ptr, [r8, +, 8*rdx, -, 32]
///      vmulpd  ymm4, ymm0, ymmword, ptr, [r8, +, 8*rdx]
///      vaddpd  ymm1, ymm1, ymmword, ptr, [rsi, +, 8*rdx, -, 96]
///      vaddpd  ymm2, ymm2, ymmword, ptr, [rsi, +, 8*rdx, -, 64]
///      vaddpd  ymm3, ymm3, ymmword, ptr, [rsi, +, 8*rdx, -, 32]
///      vaddpd  ymm4, ymm4, ymmword, ptr, [rsi, +, 8*rdx]
///      vmovupd ymmword, ptr, [rsi, +, 8*rdx, -, 96], ymm1
///      vmovupd ymmword, ptr, [rsi, +, 8*rdx, -, 64], ymm2
///      vmovupd ymmword, ptr, [rsi, +, 8*rdx, -, 32], ymm3
///      vmovupd ymmword, ptr, [rsi, +, 8*rdx], ymm4
///      add     rdx, 16
///      cmp     rbx, rdx
///      jne     .LBB130_49
/// [Reference](https://www.reidatcheson.com/matrix%20multiplication/rust/iterators/2021/02/26/gemm-iterators.html)
pub fn loop_interchange_iterators(
    a: &Array2<f64>,
    b: &Array2<f64>,
    c: &mut Array2<f64>,
    a_shape: (usize, usize),
    b_shape: (usize, usize),
) {
    let a_slice = a.as_slice().unwrap();
    let b_slice = b.as_slice().unwrap();
    let c_slice = c.as_slice_mut().unwrap();
    for (ci, ai) in c_slice
        .chunks_exact_mut(b_shape.1)
        .zip(a_slice.chunks_exact(a_shape.1))
    {
        for (aik, bk) in ai.iter().zip(b_slice.chunks_exact(b_shape.1)) {
            for (cij, bkj) in ci.iter_mut().zip(bk.iter()) {
                *cij += (*aik) * (*bkj);
            }
        }
    }
    *c = ArrayView::from_shape((a_shape.0, b_shape.1), &c_slice)
        .unwrap()
        .to_owned()
}
