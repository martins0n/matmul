import json
import pathlib
import time
from enum import Enum
from typing import Callable, Optional, Tuple

import numpy as np
import typer
from tqdm import tqdm

from src.matmul import loop_interchange_numba, naive, naive_numba, numpy_dot

MatmulCallable = Callable[
    [np.ndarray, np.ndarray, np.ndarray, Tuple[int, int], Tuple[int, int]], None
]


class MatmulMethod(str, Enum):
    naive = "naive"
    naive_numba = "naive_numba"
    loop_interchange_numba = "loop_interchange_numba"
    numpy_dot = "numpy_dot"


file_path = pathlib.Path(__file__).resolve()

log_folder = file_path.parents[2] / "logs"
exp_data_folders = sorted([i for i in (file_path.parents[2] / "data").glob("*")])


def experiment(
    matmul_impl: MatmulCallable,
    a: np.ndarray,
    b: np.ndarray,
    c: np.ndarray,
    a_shape: Tuple[int, int],
    b_shape: Tuple[int, int],
    n_iter: int,
):
    c_shape = (a_shape[0], b_shape[1])
    c_test = np.zeros(c_shape, dtype=np.float64)
    matmul_impl(a, b, c_test, a_shape, b_shape)
    np.testing.assert_allclose(c_test, a.dot(b))
    time_list = []

    for _ in tqdm(range(n_iter), desc=f"{matmul_impl.__name__}_{a.shape}_{b.shape}"):
        c_test = np.zeros(c_shape, dtype=np.float64)
        start_time = time.monotonic_ns()
        matmul_impl(a, b, c_test, a_shape, b_shape)
        time_list.append((time.monotonic_ns() - start_time) * 1e-9)

    return time_list


def run_experiments(
    matmul_impl: MatmulCallable, postfix: Optional[str], n_iter: int = 100
):
    experiment_results = dict(
        runs=list(),
        matmul_impl=f"{matmul_impl.__name__}{'' if not postfix else '_' + postfix}",
        language="python",
    )

    for data_folder in exp_data_folders[:14]:
        with open(data_folder / "params.json", "r") as f:
            params = json.load(f)
        a = np.load(data_folder / "A.npy")
        b = np.load(data_folder / "B.npy")
        c = np.load(data_folder / "C.npy")
        time_list = experiment(matmul_impl, a, b, c, a.shape, b.shape, n_iter)
        experiment_results["runs"].append(
            dict(times=time_list, run_name=params["run_name"])
        )

    with open(
        log_folder
        / f"{experiment_results['language']}-{experiment_results['matmul_impl']}.json",
        "w",
    ) as f:
        json.dump(experiment_results, f, indent=2)


def main(method: MatmulMethod, postfix: Optional[str] = None, n_iter: int = 100):
    run_experiments(globals()[method.name], postfix, n_iter)


if __name__ == "__main__":
    typer.run(main)
