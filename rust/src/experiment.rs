use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time;

use crate::matmul::MatmulImpl;
use crate::utils::read_npy;

const TOL: f64 = 1e-10_f64;

#[derive(Serialize, Deserialize, Debug)]
struct RunParams {
    run_name: String,
    a_shape: (usize, usize),
    b_shape: (usize, usize),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Experiment {
    times: Vec<f64>,
    run_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalResults {
    language: String,
    matmul_impl: String,
    runs: Vec<Experiment>,
}

impl TotalResults {
    fn new(matmul_impl: String) -> TotalResults {
        TotalResults {
            language: String::from("rust"),
            matmul_impl: matmul_impl,
            runs: Vec::new(),
        }
    }
    fn add_experiment(&mut self, experiment: Experiment) {
        self.runs.push(experiment)
    }
}

impl Experiment {
    fn new(run_name: String) -> Experiment {
        Experiment {
            run_name: run_name,
            times: Vec::new(),
        }
    }
    fn add_run_time(&mut self, run_time: f64) {
        self.times.push(run_time)
    }
}

fn experiment(
    run_name: String,
    matmul_impl: &MatmulImpl,
    a: &Array2<f64>,
    b: &Array2<f64>,
    c: &Array2<f64>,
    a_shape: (usize, usize),
    b_shape: (usize, usize),
    n_iter: i64,
) -> Experiment {
    let c_shape = (a_shape.0, b_shape.1);
    let mut c_test: Array2<f64> = Array2::zeros(c_shape);

    let gemm_to_call = matmul_impl.to_call();
    gemm_to_call(&a, &b, &mut c_test, a_shape, b_shape);
    assert_eq!((c - c_test).mapv(f64::abs).mean() < Some(TOL), true);
    let mut exp = Experiment::new(run_name.clone());

    for _ in 0..n_iter {
        let mut c_test: Array2<f64> = Array2::zeros(c_shape);
        let start_time = time::Instant::now();
        gemm_to_call(&a, &b, &mut c_test, a_shape, b_shape);
        let duration = start_time.elapsed().as_secs_f64();
        exp.add_run_time(duration);
    }
    exp
}

pub fn run_experiments(
    matmul_impl: &MatmulImpl,
    postfix: &Option<String>,
    n_iter: i64,
    path_to_data: &Path,
) -> TotalResults {
    let gem_repr = matmul_impl.to_str();
    let mut total_results = TotalResults::new(gem_repr.clone());

    for experients_folder in fs::read_dir(path_to_data).unwrap() {
        let path = experients_folder.unwrap().path();
        println!("Current path: {}", &path.display());

        let mut file = File::open(path.join("params.json")).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let params: RunParams = serde_json::from_str(&data).unwrap();

        let a = read_npy(&path.join("A.npy"), params.a_shape);
        let b = read_npy(&path.join("B.npy"), params.b_shape);
        let c = read_npy(&path.join("C.npy"), (params.a_shape.0, params.b_shape.1));

        let experient_results = experiment(
            params.run_name,
            matmul_impl,
            &a,
            &b,
            &c,
            params.a_shape,
            params.b_shape,
            n_iter,
        );

        total_results.add_experiment(experient_results);
    }

    let mut log_path = path_to_data.parent().unwrap().join("logs");
    let log_name: String = format!(
        "rust-{}{}.json",
        &gem_repr,
        postfix.clone().unwrap_or(String::from(""))
    );
    log_path.push(log_name);
    log_path.set_extension("json");
    let file = File::create(log_path).unwrap();
    let _ = serde_json::to_writer(file, &total_results).unwrap();

    total_results
}
