use crate::graphs::Graph;

#[allow(dead_code)]
pub trait LocalSearch {
    fn swap(&self, graph: &Graph, start: usize) -> Self;
    fn two_opt(&self, graph: &Graph) -> Self;
    fn shift(&self, graph: &Graph, start: usize) -> Self;
    fn or_opt(&self, graph: &Graph) -> Self;
}

#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub struct Solution {
    pub route: Vec<usize>,
    pub cost: f64,
}

#[allow(dead_code)]
impl Solution {
    pub fn calculate_cost(route: &[usize], graph: &Graph) -> f64 {
        if route.is_empty() {
            return 0.0;
        }

        route.windows(2).map(|w| graph[w[0]][w[1]]).sum::<f64>()
            + graph[route[route.len() - 1]][route[0]]
    }

    fn neighbourhood_by_swap(&self, graph: &Graph, start: usize) -> Vec<Self> {
        let mut solutions: Vec<Solution> = Vec::new();

        for v in &self.route {
            let mut new_route = self.route.clone();

            new_route.swap(start, *v);
            // let tmp = new_route[start];
            // new_route[start] = new_route[*v];
            // new_route[*v] = tmp;

            let cost = Self::calculate_cost(&new_route, graph);

            solutions.push(Self {
                route: new_route,
                cost,
            });
        }
        solutions
    }

    fn neighbourhood_by_shift(&self, graph: &Graph, start: usize) -> Vec<Self> {
        let mut solutions: Vec<Solution> = Vec::new();
        let n = self.route.len();

        if start >= n {
            return solutions;
        }

        for target_pos in 0..n {
            if target_pos == start {
                continue;
            }

            let mut new_route = self.route.clone();
            let elem = new_route.remove(start);
            new_route.insert(target_pos, elem);

            let cost = Self::calculate_cost(&new_route, graph);

            solutions.push(Self {
                route: new_route,
                cost,
            });
        }

        solutions
    }

    fn neighbourhood_by_or_opt(&self, graph: &Graph) -> Vec<Self> {
        let n = self.route.len();
        let mut neighbours = Vec::new();

        for seq_len in 1..=3.min(n) {
            for i in 0..n {
                if i + seq_len > n {
                    break;
                }

                for target in 0..n {
                    if target >= i && target < i + seq_len {
                        continue;
                    }

                    let mut new_route = self.route.clone();
                    let sequence: Vec<usize> = new_route.drain(i..(i + seq_len)).collect();

                    let insert_pos = if target > i { target - seq_len } else { target };

                    if insert_pos <= new_route.len() {
                        new_route.splice(insert_pos..insert_pos, sequence);

                        let cost = Self::calculate_cost(&new_route, graph);

                        neighbours.push(Solution {
                            route: new_route,
                            cost,
                        });
                    }
                }
            }
        }

        neighbours
    }
}

impl LocalSearch for Solution {
    fn swap(&self, graph: &Graph, start: usize) -> Self {
        let mut best_solution: Solution = self.clone();
        let mut found_better_solution = true;

        while found_better_solution {
            found_better_solution = false;
            let solutions = best_solution.neighbourhood_by_swap(graph, start);

            let mut tmp_solution = best_solution.clone();

            for s in solutions.iter() {
                if s.cost < tmp_solution.cost {
                    tmp_solution = s.clone();
                }
            }

            if tmp_solution.cost < best_solution.cost {
                best_solution = tmp_solution;
                found_better_solution = true;
            }
        }

        best_solution
    }

    fn two_opt(&self, _graph: &Graph) -> Self {
        self.clone()
    }

    fn shift(&self, graph: &Graph, start: usize) -> Self {
        let mut best_solution: Solution = self.clone();
        let mut found_better_solution = true;

        while found_better_solution {
            found_better_solution = false;

            let solutions = best_solution.neighbourhood_by_shift(graph, start);

            let mut tmp_solution = best_solution.clone();

            for s in solutions.iter() {
                if s.cost < tmp_solution.cost {
                    tmp_solution = s.clone();
                }
            }

            if tmp_solution.cost < best_solution.cost {
                best_solution = tmp_solution;
                found_better_solution = true;
            }
        }
        best_solution
    }

    fn or_opt(&self, graph: &Graph) -> Self {
        let mut best_solution: Solution = self.clone();
        let mut found_better_solution = true;

        while found_better_solution {
            found_better_solution = false;

            let solutions = best_solution.neighbourhood_by_or_opt(graph);

            let mut tmp_solution = best_solution.clone();

            for s in solutions.iter() {
                if s.cost < tmp_solution.cost {
                    tmp_solution = s.clone();
                }
            }

            if tmp_solution.cost < best_solution.cost {
                best_solution = tmp_solution;
                found_better_solution = true;
            }
        }

        best_solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INF: f64 = f64::INFINITY;

    #[test]
    fn swap_test_1() {
        let graph = vec![
            vec![INF, 1.0, 2.0, 4.0, 3.0],
            vec![1.0, INF, 7.0, 2.0, 5.0],
            vec![2.0, 7.0, INF, 8.0, 1.0],
            vec![4.0, 2.0, 8.0, INF, 6.0],
            vec![3.0, 5.0, 1.0, 6.0, INF],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25.0,
        };

        solution = solution.swap(&graph, 0);
        assert_eq!(solution.cost, 12.0);
    }

    #[test]
    fn swap_test_2() {
        let graph = vec![
            vec![INF, 1.0, 2.0, 4.0, 3.0],
            vec![1.0, INF, 7.0, 2.0, 5.0],
            vec![2.0, 7.0, INF, 8.0, 1.0],
            vec![4.0, 2.0, 8.0, INF, 6.0],
            vec![3.0, 5.0, 1.0, 6.0, INF],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 3, 4, 2],
            cost: 12.0,
        };

        solution = solution.swap(&graph, 0);
        assert_eq!(solution.cost, 12.0);
    }

    #[test]
    fn shift_test_1() {
        let graph = vec![
            vec![INF, 1.0, 2.0, 4.0, 3.0],
            vec![1.0, INF, 7.0, 2.0, 5.0],
            vec![2.0, 7.0, INF, 8.0, 1.0],
            vec![4.0, 2.0, 8.0, INF, 6.0],
            vec![3.0, 5.0, 1.0, 6.0, INF],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25.0,
        };

        solution = solution.shift(&graph, 2);
        assert_eq!(solution.cost, 12.0);
    }

    #[test]
    fn shift_test_2() {
        let graph = vec![
            vec![INF, 1.0, 2.0, 4.0, 3.0],
            vec![1.0, INF, 7.0, 2.0, 5.0],
            vec![2.0, 7.0, INF, 8.0, 1.0],
            vec![4.0, 2.0, 8.0, INF, 6.0],
            vec![3.0, 5.0, 1.0, 6.0, INF],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 3, 4, 2],
            cost: 12.0,
        };

        solution = solution.shift(&graph, 0);
        assert_eq!(solution.cost, 12.0);
    }

    #[test]
    fn or_opt_test_1() {
        let graph = vec![
            vec![INF, 1.0, 2.0, 4.0, 3.0],
            vec![1.0, INF, 7.0, 2.0, 5.0],
            vec![2.0, 7.0, INF, 8.0, 1.0],
            vec![4.0, 2.0, 8.0, INF, 6.0],
            vec![3.0, 5.0, 1.0, 6.0, INF],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25.0,
        };

        solution = solution.or_opt(&graph);
        assert!(solution.cost <= 12.0);
    }

    #[test]
    fn or_opt_test_2() {
        let graph = vec![
            vec![INF, 1.0, 2.0, 4.0, 3.0],
            vec![1.0, INF, 7.0, 2.0, 5.0],
            vec![2.0, 7.0, INF, 8.0, 1.0],
            vec![4.0, 2.0, 8.0, INF, 6.0],
            vec![3.0, 5.0, 1.0, 6.0, INF],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 3, 4, 2],
            cost: 12.0,
        };

        solution = solution.or_opt(&graph);
        assert_eq!(solution.cost, 12.0);
    }
}
