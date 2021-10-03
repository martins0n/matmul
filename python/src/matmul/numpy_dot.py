from typing import Tuple

import numpy as np


def numpy_dot(
    a: np.ndarray,
    b: np.ndarray,
    c: np.ndarray,
    a_shape: Tuple[int, int],
    b_shape: Tuple[int, int],
):
    np.matmul(a, b, out=c)
