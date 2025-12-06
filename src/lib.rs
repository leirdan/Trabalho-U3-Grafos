//! Estruturas de dados e algoritmos para Grafos.
//!
//! Crate que fornece algoritmos e estruturas de dados para Grafos.
//!
//! A organização do crate é feita em diversos módulos:
//! - `adjacency_list`: guarda a representação de um grafo no formato da Lista de Adjacência,
//!   bastante popular e comum de implementar;
//! - `traversal`: armazena os algoritmos para a travessia em um grafo. Os algoritmos incluem a
//!   BFS, DFS, DFS com classificação de arestas e identificação de componentes.

#![feature(impl_trait_in_assoc_type)]

mod adjacency_matrix;
mod graph;
mod heuristics;
mod traversal;

pub use graph::Graph;
pub use graph::Node;
pub use graph::UndirectedGraph;
pub use graph::WeightedGraph;

pub mod graphs {
    pub use crate::adjacency_matrix::AdjacencyMatrix;
    pub use crate::heuristics::nearest_neighbour;
    pub use crate::traversal::BfsEvent;
    pub use crate::traversal::BfsIter;
    pub use crate::traversal::BiconnectedComponentsIter;
    pub use crate::traversal::DfsEdgesIter;
    pub use crate::traversal::DfsEvent;
    pub use crate::traversal::DfsIter;
    pub use crate::traversal::Edge;
    pub const MAX: usize = usize::MAX;
}
