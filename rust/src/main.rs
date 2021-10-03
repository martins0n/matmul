use clap::Clap;
use std::path::Path;

mod experiment;
mod matmul;
mod utils;

#[derive(Clap)]
#[clap(setting = clap::AppSettings::ColoredHelp, name = "matmul")]
struct Opts {
    #[clap(arg_enum)]
    matmul_impl: matmul::MatmulImpl,
    #[clap(long)]
    postfix: Option<String>,
    #[clap(long, default_value = "100")]
    n_iter: i64,
    #[clap(long, default_value = "../data")]
    data_path: String,
}

fn main() {
    let args: Opts = Opts::parse();

    experiment::run_experiments(
        &args.matmul_impl,
        &args.postfix,
        args.n_iter,
        Path::new(&args.data_path),
    );
}
