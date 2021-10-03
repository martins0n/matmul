import random
from typing import Tuple
import numpy as np
from random import seed
from typing import Tuple
import pathlib
import json



SEED = 1
seed(SEED)
np.random.seed(SEED)

CURRENT_PATH = pathlib.Path(__file__)


LIST_OF_SIZES = [((2**(i+j-1), 2**i), (2**i, 2**(i+j-1))) for i in range(2,11) for j in range(2)]


def matrix_generator(size: Tuple) -> np.ndarray:
    return np.random.normal(size=size)

def matrix_saving(path: pathlib.Path, matrix: np.ndarray):
    np.save(path, matrix)


def data_prep():
    for idx, (size_a, size_b) in enumerate(LIST_OF_SIZES):
        run_name = f"n_{idx:03d}_shapes_{size_a[0]}_{size_a[1]}_{size_b[0]}_{size_b[1]}"
        folder_to_save = CURRENT_PATH.parents[1] / "data" / run_name
        folder_to_save.mkdir(exist_ok=True)
        A = matrix_generator(size_a)
        B = matrix_generator(size_b)
        C = A @ B
        matrix_saving(folder_to_save / "A", A)
        matrix_saving(folder_to_save / "B", B)
        matrix_saving(folder_to_save / "C", C)
        params = {"run_name": run_name, "a_shape": list(A.shape), "b_shape": list(B.shape)}
        with open(folder_to_save / "params.json", "w") as f:
            json.dump(params, f)


if __name__ == "__main__":
    data_prep()