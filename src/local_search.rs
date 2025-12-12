#[allow(dead_code)]
pub trait LocalSearch<Graph> {
    fn swap(&self, graph: &Graph, start: usize) -> Self;
    fn two_opt(&self, graph: &Graph) -> Self;
    fn shift(&self, graph: &Graph, start: usize) -> Self;
    fn or_opt(&self, graph: &Graph) -> Self;
}

#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub struct Solution<const N: usize> {
    pub route: Vec<usize>,
    pub cost: f64,
}

#[allow(dead_code)]
impl<const N: usize> Solution<N> {
    pub fn calculate_cost(route: &[usize], graph: &[[f64; N]; N]) -> f64 {
        if route.is_empty() {
            return 0.0;
        }

        route.windows(2).map(|w| graph[w[0]][w[1]]).sum::<f64>()
            + graph[route[route.len() - 1]][route[0]]
    }

    fn neighbourhood_by_swap(&self, graph: &[[f64; N]; N], start: usize) -> Vec<Self> {
        let mut solutions: Vec<Solution<N>> = Vec::new();

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

    fn neighbourhood_by_shift(&self, graph: &[[f64; N]; N], start: usize) -> Vec<Self> {
        let mut solutions: Vec<Solution<N>> = Vec::new();
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

    fn neighbourhood_by_or_opt(&self, graph: &[[f64; N]; N]) -> Vec<Self> {
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

impl<const N: usize> LocalSearch<[[f64; N]; N]> for Solution<N> {
    fn swap(&self, graph: &[[f64; N]; N], start: usize) -> Self {
        let mut best_solution: Solution<N> = self.clone();
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

    fn two_opt(&self, graph: &[[f64; N]; N]) -> Self {
        let n = graph.len();
        let mut current_solution: Solution<N> = self.clone();

        'outer: for i in 0..(n - 2) {
            for j in i + 2..n {
                let mut new_route: Vec<usize> = Vec::with_capacity(n);

                new_route.extend_from_slice(&self.route[0..=i]);
                new_route.extend(self.route[i + 1..=j].iter().rev());

                if j + 1 < n {
                    new_route.extend_from_slice(&self.route[j + 1..]);
                }

                let new_cost = Self::calculate_cost(&new_route, graph);
                if new_cost < current_solution.cost {
                    current_solution = Solution {
                        route: new_route,
                        cost: new_cost,
                    };
                    break 'outer;
                }
            }
        }
        current_solution
    }

    fn shift(&self, graph: &[[f64; N]; N], start: usize) -> Self {
        let mut best_solution: Solution<N> = self.clone();
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

    fn or_opt(&self, graph: &[[f64; N]; N]) -> Self {
        let mut best_solution: Solution<N> = self.clone();
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
