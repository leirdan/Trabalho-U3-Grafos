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

mod heuristics;

pub mod graphs {
    pub use crate::heuristics::nearest_neighbour;
}
