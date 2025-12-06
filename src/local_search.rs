enum SearchType {
    BestImp,
    FirstImp,
}

trait LocalSearch {
    fn swap(&self, graph: &Vec<Vec<usize>>, imp: SearchType) -> Self;
    fn two_opt(&self, graph: &Vec<Vec<usize>>, imp: SearchType) -> Self;
}

#[derive(Clone)]
struct Solution {
    route: Vec<usize>,
    cost: usize,
}

impl Solution {
    fn neighbourhood_by_swap(&self, graph: &Vec<Vec<usize>>) -> Vec<Self> {
        todo!()
    }
}

impl LocalSearch for Solution {
    fn swap(&self, graph: &Vec<Vec<usize>>, imp: SearchType) -> Self {
        let (route, cost) = (self.route.clone(), self.cost);

        let mut best_solution: &Solution = &Self { route, cost };
        let mut found_better_solution = true;
        let solutions = self.neighbourhood_by_swap(&graph);

        while found_better_solution {
            found_better_solution = false;

            match imp {
                SearchType::FirstImp => {
                    for s in solutions.iter() {
                        if s.cost < best_solution.cost {
                            found_better_solution = true;
                            best_solution = s;
                            break;
                        }
                    }
                }
                SearchType::BestImp => {
                    let mut tmp_solution = best_solution;

                    for s in solutions.iter() {
                        if s.cost < tmp_solution.cost {
                            tmp_solution = s;
                        }
                    }

                    if tmp_solution.cost < best_solution.cost {
                        best_solution = tmp_solution;
                        found_better_solution = true;
                    }
                }
            }
        }
        best_solution.clone()
    }

    fn two_opt(&self, graph: &Vec<Vec<usize>>, imp: SearchType) -> Self {
        self.clone()
    }
}
