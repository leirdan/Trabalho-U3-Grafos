use crate::graphs::MAX;

/// Implementação da Heurística do Vizinho Mais Próximo, um algoritmo guloso que gera um caminho
/// para o Problema do Caixeiro Viaijante.
///
/// # Argumentos
/// - `graph`: um grafo representado em Matriz de Adjacência;
/// - `start`: o nó inicial por onde iniciará o caminho.
///
/// # Saída
/// - Uma dupla composta pelo `path` (um vetor de vértices) e pelo `cost` (custo total da solução).
///
/// # Detalhes de funcionamento
/// 1. Criação de um vetor de booleanos `visited`, onde podemos descobrir se um vértice foi visitado
///     com complexidade O(1), e marcamos todos os vértices como "não visitados".
/// 2. Marcação do vértice inicial como visitado.
/// 3. Criação de um contador que marca a quantidade total de visitados. É uma maneira barata de determinar o fim do loop
///     em comparação a uma função de checagem por visitados de complexidade O(n).
/// 4. Enquanto houverem vértices não visitados, itera sobre cada vértice não visitado da matriz
///     à procura da aresta de menor custo. Ao encontrá-la, adiciona o vértice adjacente dessa aresta no caminho,
///     marca-o como visitado e incrementa o contador.
/// 5. Ao fim do loop, contabiliza o custo da aresta do vértice final do caminho até o vértice inicial, sem adicionar
///     novamente o inicial no caminho.
/// 6. Retorna o caminho encontrado e o custo total dele.
/// - Este algoritmo não insere ao fim do caminho o vértice inicial, pois é assumido que existe um
///     ciclo hamiltoniano ao fim deste caminho, logo, não é uma informação relevante.
pub fn nearest_neighbour(graph: Vec<Vec<usize>>, start: usize) -> (Vec<usize>, usize) {
    // Sem muitas abstrações em razão de uma suposta eficiência!!

    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut path: Vec<usize> = Vec::new();
    path.push(start);

    let mut cost = 0;
    let mut current_node = start;
    visited[current_node] = true;

    let mut visited_count = 1;

    let mut better_cost;
    let mut next_on_path: Option<usize> = None;

    while visited_count != graph.len() {
        better_cost = MAX;

        for (i, val) in visited.iter().enumerate() {
            if *val || current_node == i {
                continue;
            }

            if graph[current_node][i] < better_cost {
                better_cost = graph[current_node][i];
                next_on_path = Some(i);
            }
        }

        if let Some(next_on_path) = next_on_path {
            let n = next_on_path;
            path.push(n);
            cost += better_cost;
            current_node = n;
            visited[n] = true;
            visited_count += 1;
        }
    }

    cost += graph[current_node][start];

    // Adicionar ou não novamente o vértice inicial? Matheus comentou que não era interessante...

    (path, cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let graph = vec![
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

    #[test]
    fn test_2() {
        let graph = vec![
            vec![MAX, 1, 3, 6],
            vec![1, MAX, 2, 3],
            vec![3, 2, MAX, 1],
            vec![6, 3, 1, MAX],
        ];

        let (path, cost) = nearest_neighbour(graph, 0);

        assert_eq!(cost, 10);
        assert_eq!(path[0], 0);
        assert_eq!(path.last(), Some(3).as_ref());
    }

    #[test]
    fn test_3() {
        let graph = vec![
            vec![MAX, 1, 3, 1000],
            vec![1, MAX, 2, 3],
            vec![3, 2, MAX, 1],
            vec![1000, 3, 1, MAX],
        ];

        let (path, cost) = nearest_neighbour(graph, 0);

        assert_eq!(cost, 1004);
        assert_eq!(path[0], 0);
        assert_eq!(path.last(), Some(3).as_ref());
    }
}
