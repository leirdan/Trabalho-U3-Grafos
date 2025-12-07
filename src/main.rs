#![feature(slice_swap_unchecked)]

use std::{array, time::Instant};

use csv_macro::graph_from_csv;
use rand::{rngs::ThreadRng, seq::SliceRandom};

graph_from_csv!("data/grasp-min/data.csv");

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

/// Executes a crossover between the individuals of two equal halves of a population and
/// possibly mutates each offspring.
///
/// The crossover operation is the Sequential Constructive Crossover (SCX) from Ahmed Z.
/// "Genetic Algorithm for the Traveling Salesman Problem using Sequential Constructive Crossover
/// Operator". The mutation is a simple swap of a random contiguous pair in the offspring. After
/// the crossover+mutation operation, the population is reshuffled to enable the crossover of
/// different individuals.
#[inline]
fn love(rng: &mut ThreadRng, mrate: f64, p: &mut Population) {
    let (h1, h2) = p.split_at_mut(p.len() / 2);
    for (p1, p2) in h1.iter_mut().zip(h2) {
        if let Some(i) = cross(p1, p2)
            && rand::random_bool(mrate)
        {
            mutate([p1, p2][i]);
        }
    }
    p.shuffle(rng);
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

    // Make love.
    for _ in 0..itnum {
        love(&mut rng, mrate, p);
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
