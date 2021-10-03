from typing import Tuple

import numba
import numpy as np


@numba.jit(nopython=True)
def loop_interchange_numba(
    a: np.ndarray,
    b: np.ndarray,
    c: np.ndarray,
    a_shape: Tuple[int, int],
    b_shape: Tuple[int, int],
):
    """vec_one: interchange loops
    Reference: http://web.mit.edu/neboat/www/6.S898-sp17/mm.pdf
    """
    for i in range(a_shape[0]):
        for j in range(a_shape[1]):
            for k in range(b_shape[1]):
                c[i, k] += a[i, j] * b[j, k]
    return c
