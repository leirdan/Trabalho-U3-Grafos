#![feature(slice_swap_unchecked)]

use std::array;

use csv_macro::graph_from_csv;
use rand::{rngs::ThreadRng, seq::SliceRandom};

graph_from_csv!("problem.csv");

const MAX_PSIZE: usize = 200;

type Fit = f64;
type Individual = [Node; NODE_COUNT];
type Population = [Individual];

#[inline]
fn init_pop(rng: &mut ThreadRng, p: &mut Population) {
    let mut rit = 0..NODE_COUNT;
    // SAFETY: Range iterator was declared above with NODE_COUNT.
    let mut r: [usize; NODE_COUNT] = array::from_fn(|_| unsafe { rit.next().unwrap_unchecked() });
    for i in p {
        r.shuffle(rng);
        *i = r;
    }
}

#[inline]
fn fit(i: &Individual) -> Fit {
    i.windows(2).map(|w| g[w[0]][w[1]]).sum::<Fit>() + g[i[NODE_COUNT - 1]][i[0]]
}

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

#[inline]
fn cross(p1: &mut Individual, p2: &mut Individual) -> Option<usize> {
    let mut offspring = [0; NODE_COUNT];
    let mut visited = [false; NODE_COUNT];
    let mut start = [&p1, &p2][rand::random_range(0..2)][0];
    offspring[0] = start;
    visited[start] = true;
    for i in offspring.iter_mut().skip(1) {
        let a = legitimate(start, &mut visited, p1);
        let b = legitimate(start, &mut visited, p2);
        *i = if g[start][a] < g[start][b] { a } else { b };
        start = *i;
        visited[*i] = true;
    }
    [p1, p2].iter_mut().enumerate().find_map(|(i, p)| {
        (fit(&offspring) < fit(p)).then(|| {
            **p = offspring;
            i
        })
    })
}

fn legitimate(start: usize, visited: &mut [bool; NODE_COUNT], i: &Individual) -> Node {
    if let Some(n) = i.iter().enumerate().find_map(|(k, n)| {
        if *n == start {
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

#[inline]
fn mutate(x: &mut Individual) {
    let pos = rand::random_range(0..NODE_COUNT - 1);
    // SAFETY: pos is at least NODE_COUNT - 1 and NODE_COUNT is greater than 1.
    unsafe {
        x.swap_unchecked(pos, pos + 1);
    }
}

fn main() {
    let mut rng = rand::rng();

    // Load hyperparams.
    let args = std::env::args().collect::<Vec<_>>();
    let itnum: usize = args[1].parse().unwrap();
    let psize: usize = args[2].parse().unwrap();
    let mrate: f64 = args[3].parse().unwrap();
    assert!(psize < MAX_PSIZE);

    // Init population.
    let p: &mut Population = &mut [[0; NODE_COUNT]; MAX_PSIZE][..psize];
    init_pop(&mut rng, p);

    // Make love.
    for _ in 0..itnum {
        love(&mut rng, mrate, p);
    }

    // Print best fitness.
    println!(
        "{}",
        p.iter()
            .map(fit)
            .min_by(|x, y| x.total_cmp(y))
            .unwrap_or(f64::INFINITY)
    )
}
