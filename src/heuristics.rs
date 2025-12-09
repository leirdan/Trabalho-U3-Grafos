/// Implementação da Heurística do Vizinho Mais Próximo, um algoritmo guloso que gera um caminho
use crate::local_search::Solution;

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
pub fn nearest_neighbour(graph: &Vec<Vec<usize>>, start: usize) -> Solution {
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
        better_cost = usize::MAX;

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
///       custo_extra = d(u, r*) + d(r*, v) − d(u, v).
/// 6. O vértice `r*` é então inserido na melhor posição do ciclo e marcado como pertencente ao ciclo.
/// 7. O vetor `min_dist` é atualizado em tempo O(n), ajustando as distâncias mínimas dos vértices
///    ainda não inseridos.
/// 8. O processo continua até que todos os vértices estejam presentes no ciclo.
pub fn nearest_insertion(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut in_cycle = vec![false; n];
    let mut min_dist = vec![MAX; n];
    in_cycle[start] = true;

    let mut first: Option<usize> = None;
    let mut best = MAX;

    for i in 0..n {
        if i != start && graph[start][i] < best {
            best = graph[start][i];
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
        let mut best_dist = MAX;

        for v in 0..n {
            if !in_cycle[v] && min_dist[v] < best_dist {
                best_dist = min_dist[v];
                r_star = Some(v);
            }
        }

        let r_star = r_star.expect("No candidate vertex found");
        let mut best_extra = MAX;
        let mut best_pos = 0;

        for i in 0..cycle.len() - 1 {
            let u = cycle[i];
            let v = cycle[i + 1];

            let du = graph[u][r_star];
            let dv = graph[r_star][v];
            let uv = graph[u][v];

            let extra = du.saturating_add(dv).saturating_sub(uv);

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
    cycle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let solution = nearest_neighbour(&graph, 0);

        assert_eq!(solution.cost, 12);
        assert_eq!(solution.route[0], 0);
        assert_eq!(solution.route.last(), Some(2).as_ref());
    }

    #[test]
    fn test_2() {
        let graph = vec![
            vec![usize::MAX, 1, 3, 6],
            vec![1, usize::MAX, 2, 3],
            vec![3, 2, usize::MAX, 1],
            vec![6, 3, 1, usize::MAX],
        ];

        let solution = nearest_neighbour(&graph, 0);

        assert_eq!(solution.cost, 10);
        assert_eq!(solution.route[0], 0);
        assert_eq!(solution.route.last(), Some(3).as_ref());
    }

    #[test]
    fn test_3() {
        let graph = vec![
            vec![usize::MAX, 1, 3, 1000],
            vec![1, usize::MAX, 2, 3],
            vec![3, 2, usize::MAX, 1],
            vec![1000, 3, 1, usize::MAX],
        ];

        let solution = nearest_neighbour(&graph, 0);

        assert_eq!(solution.cost, 1004);
        assert_eq!(solution.route[0], 0);
        assert_eq!(solution.route.last(), Some(3).as_ref());
    }

    fn calc_cost(graph: &Vec<Vec<usize>>, path: &Vec<usize>) -> usize {
        let mut cost = 0;
        for i in 0..path.len() - 1 {
            cost += graph[path[i]][path[i + 1]];
        }
        cost
    }

    fn is_valid_cycle(path: &Vec<usize>, n: usize) -> bool {
        if path.len() != n + 1 {
            return false;
        }
        if path.first() != path.last() {
            return false;
        }
        let mut visited = vec![false; n];
        for &v in path.iter().take(path.len() - 1) {
            if v >= n || visited[v] {
                return false;
            }
            visited[v] = true;
        }

        true
    }

    #[test]
    fn test_tsp_1() {
        let graph = vec![
            vec![usize::MAX, 10, 15, 20],
            vec![10, usize::MAX, 35, 25],
            vec![15, 35, usize::MAX, 30],
            vec![20, 25, 30, usize::MAX],
        ];

        let path = nearest_insertion(&graph, 0);
        let cost = calc_cost(&graph, &path);

        assert!(is_valid_cycle(&path, graph.len()));
        assert!(cost > 0);
    }

    #[test]
    fn test_tsp_2() {
        let graph = vec![
            vec![usize::MAX, 2, 9, 10, 7],
            vec![2, usize::MAX, 6, 4, 3],
            vec![9, 6, usize::MAX, 8, 5],
            vec![10, 4, 8, usize::MAX, 6],
            vec![7, 3, 5, 6, usize::MAX],
        ];

        let path = nearest_insertion(&graph, 0);
        let cost = calc_cost(&graph, &path);

        assert!(is_valid_cycle(&path, graph.len()));
        assert!(cost > 0);
    }

    #[test]
    fn test_tsp_3() {
        let graph = vec![
            vec![usize::MAX, 1, 50, 100, 1],
            vec![1, usize::MAX, 1, 50, 100],
            vec![50, 1, usize::MAX, 1, 50],
            vec![100, 50, 1, usize::MAX, 1],
            vec![1, 100, 50, 1, usize::MAX],
        ];

        let path = nearest_insertion(&graph, 0);
        let cost = calc_cost(&graph, &path);

        assert!(is_valid_cycle(&path, graph.len()));
        assert!(cost > 0);
    }
}
