use ndarray::Array2;
use ndarray::ArrayView;

/// Matmul using iterators
/// asm:
///     .LBB130_49:
///         *cij += (*aik) * (*bkj);
///         movupd  xmm1, xmmword, ptr, [r8, +, 8*rdx, -, 16]
///         movupd  xmm2, xmmword, ptr, [r8, +, 8*rdx]
///         mulpd   xmm1, xmm0
///         mulpd   xmm2, xmm0
///         *cij += (*aik) * (*bkj);
///         movupd  xmm3, xmmword, ptr, [rsi, +, 8*rdx, -, 16]
///         addpd   xmm3, xmm1
///         movupd  xmm1, xmmword, ptr, [rsi, +, 8*rdx]
///         addpd   xmm1, xmm2
///         movupd  xmmword, ptr, [rsi, +, 8*rdx, -, 16], xmm3
///         movupd  xmmword, ptr, [rsi, +, 8*rdx], xmm1
///         add     rdx, 4
///         cmp     rbx, rdx
///         jne     .LBB130_49
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
