from typing import Tuple

import numba
import numpy as np


@numba.jit(nopython=True)
def naive_numba(
    a: np.ndarray,
    b: np.ndarray,
    c: np.ndarray,
    a_shape: Tuple[int, int],
    b_shape: Tuple[int, int],
):
    """a_{ij}*b_{jk}"""
    for i in range(a_shape[0]):
        for k in range(b_shape[1]):
            c[i, k] = 0
            for j in range(a_shape[1]):
                c[i, k] += a[i, j] * b[j, k]
    return c
