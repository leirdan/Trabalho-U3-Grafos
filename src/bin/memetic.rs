#![feature(slice_swap_unchecked)]

use csv_macro::graph_from_csv;
use graphs_algorithms::local_search::LocalSearch;
use graphs_algorithms::local_search::Solution;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{array, time::Instant};

graph_from_csv!("data/012/data.csv");

/// A metric for representing the quality of a solution to the TSP problem.
type Fit = f64;

/// A candidate solution for the TSP problem.
type Individual = [Node; NODE_COUNT];

/// Pool of current candidate solutions.
type Population = [Individual];

/// Fills a population with random solutions (individuals).
#[inline]
fn init(rng: &mut ThreadRng, p: &mut Population) {
    let mut rit = 0..NODE_COUNT;
    // SAFETY: Range iterator was declared above with NODE_COUNT.
    let mut r: [usize; NODE_COUNT] = array::from_fn(|_| unsafe { rit.next().unwrap_unchecked() });
    for i in p {
        r.shuffle(rng);
        *i = r;
    }
}

/// Maps a fitness for some individual.
///
/// In this case, the sum of edge costs between adjacent nodes in the individual, including a cycle
/// back to the beginning.
#[inline]
fn fit(i: &Individual) -> Fit {
    i.windows(2).map(|w| g[w[0]][w[1]]).sum::<Fit>() + g[i[NODE_COUNT - 1]][i[0]]
}

/// Executes a Sequential Constructive Crossover (SCX) between two individuals and overwrites a
/// parent with the offspring if the new individual is more fit. Returns the position of the
/// overwritten parent in the `[p1, p2]` array.
#[inline]
fn cross(p1: &mut Individual, p2: &mut Individual) -> Option<usize> {
    let mut offspring = [0; NODE_COUNT];
    let mut visited = [false; NODE_COUNT];
    let mut fst = [&p1, &p2][rand::random_range(0..2)][0];
    offspring[0] = fst;
    visited[fst] = true;
    for n in offspring.iter_mut().skip(1) {
        let a = legitimate(fst, &mut visited, p1);
        let b = legitimate(fst, &mut visited, p2);
        *n = if g[fst][a] < g[fst][b] { a } else { b };
        fst = *n;
        visited[*n] = true;
    }
    [p1, p2].iter_mut().enumerate().find_map(|(i, p)| {
        (fit(&offspring) < fit(p)).then(|| {
            **p = offspring;
            i
        })
    })
}

/// Helper function to find the first _legitimate node_ after `fst` in the crossover operation ([`cross`]).
fn legitimate(fst: usize, visited: &mut [bool; NODE_COUNT], i: &Individual) -> Node {
    if let Some(n) = i.iter().enumerate().find_map(|(k, n)| {
        if *n == fst {
            i[k + 1..].iter().find(|n| !visited[**n])
        } else {
            None
        }
    }) {
        *n
    } else {
        (0..NODE_COUNT).find(|n| !visited[*n]).unwrap()
    }
}

/// Executes a swap operation in a random contiguous pair of nodes in the individual.
#[inline]
fn mutate(i: &mut Individual) {
    let pos = rand::random_range(0..NODE_COUNT - 1);
    // SAFETY: pos is at least NODE_COUNT - 1 and NODE_COUNT is greater than 1.
    unsafe {
        i.swap_unchecked(pos, pos + 1);
    }
}

/// The maximum size that the population can have.
const MAX_PSIZE: usize = 200;

/// Função utilitária que converte um Individual para uma Solution a fim de utilizar as buscas locais implementadas.
fn individual_to_solution(p: &Individual) -> Solution<NODE_COUNT> {
    let mut s = Solution {
        route: vec![],
        cost: 0.0,
    };
    s.route = p.to_vec();
    s.cost = Solution::calculate_cost(&s.route, &g);
    s
}

/// Similar à `love`, executa um cruzamento entre pares de indivíduos e possivelmente uma mutação.
/// A diferença em relação à `love` original está após a mutação, onde sorteamos de forma aleatória alguma das buscas locais implementadas
/// e aplicamos no indivíduo.
/// Ao fim, a população é reorganizada aleatoriamente para permitir novos cruzamentos na próxima geração.
#[inline]
fn love_w_gen_mods(rng: &mut ThreadRng, mrate: f64, p: &mut Population) {
    let rnd_op = rand::random_range(1..=100);
    let (h1, h2) = p.split_at_mut(p.len() / 2);
    for (p1, p2) in h1.iter_mut().zip(h2) {
        if let Some(i) = cross(p1, p2)
            && rand::random_bool(mrate)
        {
            let offspring = &mut [p1, p2][i];
            let s = {
                mutate(offspring);
                individual_to_solution(offspring)
            };
            let s = match rnd_op {
                1..25 => s.shift(&g, s.route[0]),
                25..50 => s.swap(&g, s.route[0]),
                50..75 => s.two_opt(&g),
                _ => s.or_opt(&g),
            };
            offspring.copy_from_slice(&s.route);
        }
    }

    p.shuffle(rng);
}

fn main() {
    let now = Instant::now();
    let mut rng = rand::rng();

    // Load hyper-params.
    let args = std::env::args().collect::<Vec<_>>();
    let itnum: usize = args[1].parse().unwrap(); // Number of iterations (or generations).
    let psize: usize = args[2].parse().unwrap(); // Size of the population by number of individuals.
    let mrate: f64 = args[3].parse().unwrap(); // Mutation rate.
    assert!(psize <= MAX_PSIZE);

    // Init population.
    let p: &mut Population = &mut [[0; NODE_COUNT]; MAX_PSIZE][..psize];
    init(&mut rng, p);

    // Make love with some genetics manipulation.
    for _ in 0..itnum {
        love_w_gen_mods(&mut rng, mrate, p);
    }

    // Print best fitness and time taken.
    println!(
        "{} {}",
        p.iter()
            .map(fit)
            .min_by(|x, y| x.total_cmp(y))
            .unwrap_or(f64::INFINITY),
        now.elapsed().as_secs_f64()
    )
}
