#![feature(impl_trait_in_assoc_type)]

mod heuristics;
mod local_search;

pub mod graphs {
    pub use crate::heuristics::nearest_neighbour;
    pub type MatrixGraph = Vec<Vec<f64>>;
    pub use crate::local_search::Solution;
    pub use crate::local_search::LocalSearch;
}
