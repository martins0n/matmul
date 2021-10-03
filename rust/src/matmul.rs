use ndarray::Array2;

pub mod loop_interchange;
pub mod naive;

use loop_interchange::loop_interchange;
use naive::naive;

#[derive(Debug, PartialEq, clap::ArgEnum)]
pub enum MatmulImpl {
    Naive,
    LoopInterchange,
}

impl MatmulImpl {
    pub fn to_str(&self) -> String {
        match self {
            MatmulImpl::Naive => String::from("naive"),
            MatmulImpl::LoopInterchange => String::from("loop_interchange"),
        }
    }

    pub fn to_call(
        &self,
    ) -> fn(&Array2<f64>, &Array2<f64>, &mut Array2<f64>, (usize, usize), (usize, usize)) {
        match self {
            MatmulImpl::Naive => naive,
            MatmulImpl::LoopInterchange => loop_interchange,
        }
    }
}
