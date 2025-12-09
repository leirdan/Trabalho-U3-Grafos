use crate::graphs::Graph;
use crate::local_search::Solution;

/// Implementação da heurística do Vizinho Mais Próximo (Nearest Neighbor), um algoritmo
/// guloso que gera um caminho para o Problema do Caixeiro Viajante.
///
/// # Argumentos
/// - `graph`: grafo representado em matriz de adjacência;
/// - `start`: nó inicial por onde o caminho começará.
///
/// # Saída
/// Retorna uma `Solution`, com a rota encontrada e o custo dela.
///
/// # Detalhes de funcionamento
/// 1. Cria um vetor booleano `visited`, onde é possível verificar em O(1) se um
///    vértice já foi visitado. Todos começam como "não visitados".
/// 2. Marca o vértice inicial como visitado.
/// 3. Cria um contador com a quantidade de vértices visitados. Isso permite
///    determinar o fim do loop sem precisar usar uma checagem O(n).
/// 4. Enquanto houver vértices não visitados, itera sobre eles na matriz em busca
///    da aresta de menor custo. Ao encontrá-la, adiciona seu vértice adjacente ao
///    caminho, marca-o como visitado e incrementa o contador.
/// 5. Ao fim do loop, adiciona ao custo a aresta que liga o último vértice do
///    caminho ao vértice inicial, sem adicionar este novamente ao caminho.
/// 6. Retorna o caminho encontrado e o custo total.
///
/// Observação: este algoritmo **não** insere o vértice inicial no fim do caminho,
/// pois assume-se que há um ciclo hamiltoniano implícito; portanto, adicionar o
/// vértice inicial novamente não é necessário.
pub fn nearest_neighbour(graph: &Graph, start: usize) -> Solution {
    // Sem muitas abstrações em razão de uma suposta eficiência!!

    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut path: Vec<usize> = Vec::new();
    path.push(start);

    let mut cost: f64 = 0.0;
    let mut current_node = start;
    visited[current_node] = true;

    let mut visited_count = 1;

    let mut better_cost;
    let mut next_on_path: Option<usize> = None;

    while visited_count != graph.len() {
        better_cost = f64::INFINITY;

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

    Solution { route: path, cost }
}

/// Implementação da Heurística de Inserção Mais Próxima (*Nearest Insertion*), um algoritmo
/// guloso utilizado para gerar um ciclo aproximado para o Problema do Caixeiro Viajante (TSP).
///
/// # Argumentos
/// - `graph`: um grafo representado por uma matriz de adjacência, onde `graph[i][j]` indica
///   o custo (ou distância) da aresta entre os vértices `i` e `j`;
/// - `start`: o vértice inicial que será utilizado como base para a construção do ciclo.
///
/// # Saída
/// - Um vetor de vértices que representa o ciclo encontrado, iniciando e terminando no vértice `start`.
///
/// # Detalhes de funcionamento
/// 1. Cria-se um vetor de booleanos `in_cycle` para marcar se um vértice já pertence ao ciclo,
///    permitindo consultas em tempo O(1).
/// 2. Inicializa o ciclo com o vértice `start` e o seu vizinho mais próximo, formando um ciclo
///    inicial mínimo do tipo `start → v → start`.
/// 3. Cria-se um vetor auxiliar `min_dist`, que armazena, para cada vértice fora do ciclo,
///    a menor distância até qualquer vértice já presente no ciclo.
/// 4. Enquanto ainda houver vértices fora do ciclo, escolhe-se o vértice `r*` cuja distância
///    ao ciclo é mínima (critério de *nearest*).
/// 5. Para o vértice escolhido, testa-se todas as arestas consecutivas do ciclo a fim de encontrar
///    a posição de inserção que minimize o custo adicional:
///    custo_extra = d(u, r*) + d(r*, v) − d(u, v).
/// 6. O vértice `r*` é então inserido na melhor posição do ciclo e marcado como pertencente ao ciclo.
/// 7. O vetor `min_dist` é atualizado em tempo O(n), ajustando as distâncias mínimas dos vértices
///    ainda não inseridos.
/// 8. O processo continua até que todos os vértices estejam presentes no ciclo.
#[allow(dead_code)]
pub fn nearest_insertion(graph: &Graph, start: usize) -> Solution {
    let n = graph.len();
    let mut in_cycle = vec![false; n];
    let mut min_dist = vec![f64::INFINITY; n];
    in_cycle[start] = true;

    let mut first: Option<usize> = None;
    let mut best = f64::INFINITY;

    for (i, row) in graph.iter().enumerate().take(n) {
        if i != start && row[start] < best {
            best = row[start];
            first = Some(i);
        }
    }

    let first = first.expect("Invalid graph: no vertex found to start the cycle");
    in_cycle[first] = true;
    let mut cycle = vec![start, first, start];
    for v in 0..n {
        if !in_cycle[v] {
            min_dist[v] = graph[v][start].min(graph[v][first]);
        }
    }

    let mut count_in_cycle = 2;
    while count_in_cycle < n {
        let mut r_star: Option<usize> = None;
        let mut best_dist = f64::INFINITY;

        for v in 0..n {
            if !in_cycle[v] && min_dist[v] < best_dist {
                best_dist = min_dist[v];
                r_star = Some(v);
            }
        }

        let r_star = r_star.expect("No candidate vertex found");
        let mut best_extra = f64::INFINITY;
        let mut best_pos = 0;

        for i in 0..cycle.len() - 1 {
            let u = cycle[i];
            let v = cycle[i + 1];

            let du = graph[u][r_star];
            let dv = graph[r_star][v];
            let uv = graph[u][v];

            let extra = du + dv - uv;

            if extra < best_extra {
                best_extra = extra;
                best_pos = i;
            }
        }

        cycle.insert(best_pos + 1, r_star);
        in_cycle[r_star] = true;
        count_in_cycle += 1;

        for v in 0..n {
            if !in_cycle[v] {
                let d = graph[v][r_star];
                if d < min_dist[v] {
                    min_dist[v] = d;
                }
            }
        }
    }

    cycle.pop();
    let cost = Solution::calculate_cost(&cycle, graph);

    Solution { route: cycle, cost }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INF: f64 = f64::INFINITY;

    #[test]
    fn nearest_neighbour_1() {
        let graph = vec![
            vec![INF, 1.0, 2.0, 4.0, 3.0],
            vec![1.0, INF, 7.0, 2.0, 5.0],
            vec![2.0, 7.0, INF, 8.0, 1.0],
            vec![4.0, 2.0, 8.0, INF, 6.0],
            vec![3.0, 5.0, 1.0, 6.0, INF],
        ];

        let solution = nearest_neighbour(&graph, 0);

        assert_eq!(solution.cost, 12.0);
        assert_eq!(solution.route[0], 0);
        assert_eq!(solution.route.last(), Some(2).as_ref());
    }

    #[test]
    fn nearest_neighbour_2() {
        let graph = vec![
            vec![INF, 1.0, 3.0, 6.0],
            vec![1.0, INF, 2.0, 3.0],
            vec![3.0, 2.0, INF, 1.0],
            vec![6.0, 3.0, 1.0, INF],
        ];

        let solution = nearest_neighbour(&graph, 0);

        assert_eq!(solution.cost, 10.0);
        assert_eq!(solution.route[0], 0);
        assert_eq!(solution.route.last(), Some(3).as_ref());
    }

    #[test]
    fn test_3() {
        let graph = vec![
            vec![INF, 1.0, 3.0, 1000.0],
            vec![1.0, INF, 2.0, 3.0],
            vec![3.0, 2.0, INF, 1.0],
            vec![1000.0, 3.0, 1.0, INF],
        ];

        let solution = nearest_neighbour(&graph, 0);

        assert_eq!(solution.cost, 1004.0);
        assert_eq!(solution.route[0], 0);
        assert_eq!(solution.route.last(), Some(3).as_ref());
    }

    fn is_valid_cycle(route: &[usize], n: usize) -> bool {
        if route.len() != n {
            return false;
        }

        let mut visited = vec![false; n];
        for &v in route.iter() {
            if v >= n || visited[v] {
                return false;
            }
            visited[v] = true;
        }

        true
    }

    #[test]
    fn tsp_1() {
        let graph = vec![
            vec![INF, 10.0, 15.0, 20.0],
            vec![10.0, INF, 35.0, 25.0],
            vec![15.0, 35.0, INF, 30.0],
            vec![20.0, 25.0, 30.0, INF],
        ];

        let solution = nearest_insertion(&graph, 0);

        assert!(is_valid_cycle(&solution.route, graph.len()));
        assert!(solution.cost > 0.0);
    }

    #[test]
    fn tsp_2() {
        let graph = vec![
            vec![INF, 2.0, 9.0, 10.0, 7.0],
            vec![2.0, INF, 6.0, 4.0, 3.0],
            vec![9.0, 6.0, INF, 8.0, 5.0],
            vec![10.0, 4.0, 8.0, INF, 6.0],
            vec![7.0, 3.0, 5.0, 6.0, INF],
        ];

        let solution = nearest_insertion(&graph, 0);

        assert!(is_valid_cycle(&solution.route, graph.len()));
        assert!(solution.cost > 0.0);
    }

    #[test]
    fn tsp_3() {
        let graph = vec![
            vec![INF, 1.0, 50.0, 100.0, 1.0],
            vec![1.0, INF, 1.0, 50.0, 100.0],
            vec![50.0, 1.0, INF, 1.0, 50.0],
            vec![100.0, 50.0, 1.0, INF, 1.0],
            vec![1.0, 100.0, 50.0, 1.0, INF],
        ];

        let solution = nearest_insertion(&graph, 0);

        assert!(is_valid_cycle(&solution.route, graph.len()));
        assert!(solution.cost > 0.0);
    }
}
