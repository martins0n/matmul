import numpy as np
import pytest
from numpy.testing import assert_allclose

from src.matmul import loop_interchange_numba, naive, naive_numba, numpy_dot


@pytest.mark.parametrize(
    "matmul_impl", [naive, naive_numba, numpy_dot, loop_interchange_numba]
)
def test_case_one(matmul_impl):
    rng = np.random.default_rng(1)
    a = rng.normal(size=(2, 4))
    b = rng.normal(size=(4, 2))
    c = np.zeros((2, 2), dtype=np.float64)
    matmul_impl(a, b, c, (2, 4), (4, 2))
    assert_allclose(c, a.dot(b))


@pytest.mark.parametrize(
    "matmul_impl", [naive, naive_numba, numpy_dot, loop_interchange_numba]
)
def test_case_two(matmul_impl):
    rng = np.random.default_rng(1)
    a = rng.normal(size=(10, 100))
    b = rng.normal(size=(100, 20))
    c = np.zeros((10, 20), dtype=np.float64)
    matmul_impl(a, b, c, (10, 100), (100, 20))
    assert_allclose(c, a.dot(b))
