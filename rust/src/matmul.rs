use ndarray::Array2;

pub mod loop_interchange;
pub mod loop_interchange_iterators;
pub mod loop_interchange_uncheck;
pub mod naive;

use loop_interchange::loop_interchange;
use loop_interchange_iterators::loop_interchange_iterators;
use loop_interchange_uncheck::loop_interchange_uncheck;
use naive::naive;

#[derive(Debug, PartialEq, clap::ArgEnum)]
pub enum MatmulImpl {
    Naive,
    LoopInterchange,
    LoopInterchangeIterators,
    LoopInterchangeUncheck,
}

impl MatmulImpl {
    pub fn to_str(&self) -> String {
        match self {
            MatmulImpl::Naive => String::from("naive"),
            MatmulImpl::LoopInterchange => String::from("loop_interchange"),
            MatmulImpl::LoopInterchangeIterators => String::from("loop_interchange_iterators"),
            MatmulImpl::LoopInterchangeUncheck => String::from("loop_interchange_uncheck"),
        }
    }

    pub fn to_call(
        &self,
    ) -> fn(&Array2<f64>, &Array2<f64>, &mut Array2<f64>, (usize, usize), (usize, usize)) {
        match self {
            MatmulImpl::Naive => naive,
            MatmulImpl::LoopInterchange => loop_interchange,
            MatmulImpl::LoopInterchangeIterators => loop_interchange_iterators,
            MatmulImpl::LoopInterchangeUncheck => loop_interchange_uncheck,
        }
    }
}
