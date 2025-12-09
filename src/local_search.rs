use std::collections::HashSet;

enum SearchType {
    BestImp,
    FirstImp,
}

trait LocalSearch {
    fn swap(&self, graph: &Vec<Vec<usize>>, start: usize, imp: SearchType) -> Self;
    fn two_opt(&self, graph: &Vec<Vec<usize>>, imp: SearchType) -> Self;
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Solution {
    pub route: Vec<usize>,
    pub cost: usize,
}

impl Solution {
    fn neighbourhood_by_swap(&self, graph: &Vec<Vec<usize>>, start: usize) -> HashSet<Self> {
        let mut solutions: HashSet<Solution> = HashSet::new();

        for v in &self.route {
            let mut new_route = self.route.clone();

            let tmp = new_route[start];
            new_route[start] = new_route[*v];
            new_route[*v] = tmp;

            let cost = new_route
                .windows(2)
                .map(|w| graph[w[0]][w[1]])
                .sum::<usize>()
                + graph[new_route[new_route.len() - 1]][new_route[0]];

            solutions.insert(Self {
                route: new_route,
                cost,
            });
        }

        solutions
    }
}

impl LocalSearch for Solution {
    fn swap(&self, graph: &Vec<Vec<usize>>, start: usize, imp: SearchType) -> Self {
        let mut best_solution: Solution = self.clone();
        let mut found_better_solution = true;

        while found_better_solution {
            found_better_solution = false;

            let solutions = best_solution.neighbourhood_by_swap(&graph, start);

            match imp {
                SearchType::FirstImp => {
                    for s in solutions.iter() {
                        if s.cost < best_solution.cost {
                            found_better_solution = true;
                            best_solution = s.clone();
                            break;
                        }
                    }
                }
                SearchType::BestImp => {
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
            }
        }
        best_solution
    }

    fn two_opt(&self, graph: &Vec<Vec<usize>>, imp: SearchType) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_1_first_imp_test_1() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25,
        };

        solution = solution.swap(&graph, 0, SearchType::FirstImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_first_imp_test_2() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 3, 4, 2],
            cost: 12,
        };

        solution = solution.swap(&graph, 0, SearchType::FirstImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_first_imp_test_3() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25,
        };

        solution = solution.swap(&graph, 2, SearchType::FirstImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_best_imp_test_1() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 3, 4, 2],
            cost: 12,
        };

        solution = solution.swap(&graph, 0, SearchType::BestImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_best_imp_test_2() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25,
        };

        solution = solution.swap(&graph, 2, SearchType::BestImp);
        assert_eq!(solution.cost, 12);
    }
}
