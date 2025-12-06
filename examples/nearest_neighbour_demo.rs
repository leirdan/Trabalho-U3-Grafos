use graphs_algorithms::graphs::{MAX, nearest_neighbour};

fn main() {
    let mut graph = vec![
        vec![MAX, 1, 2, 4, 3],
        vec![1, MAX, 7, 2, 5],
        vec![2, 7, MAX, 8, 1],
        vec![4, 2, 8, MAX, 6],
        vec![3, 5, 1, 6, MAX],
    ];

    let (path, cost) = nearest_neighbour(graph, 0);

    assert_eq!(cost, 12);
    assert_eq!(path[0], 0);
    assert_eq!(path.last(), Some(2).as_ref());
}
