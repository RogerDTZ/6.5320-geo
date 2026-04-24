use clap::Parser;
use rand::prelude::*;
use rand::seq::index::sample;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    d: usize,
    n: usize,
    m: usize,
    k: usize,
    l: usize,
    r: usize,

    #[arg(long, default_value_t = false)]
    mutate_dup: bool,

    #[arg(long, short, default_value_t = 0)]
    threads: usize,
}

struct ANNResult {
    correct: f64,
}

fn hamming_distance(p: &Vec<bool>, q: &Vec<bool>) -> usize {
    p.iter().zip(q.iter()).filter(|(a, b)| a != b).count()
}

fn random_indices(rng: &mut ThreadRng, n: usize, k: usize, dup: bool) -> Vec<usize> {
    if dup {
        (0..k).map(|_| rng.random_range(0..n)).collect()
    } else {
        sample(rng, n, k).into_vec()
    }
}

fn generate_points(rng: &mut ThreadRng, n: usize, d: usize) -> Vec<Vec<bool>> {
    (0..n)
        .map(|_| (0..d).map(|_| rng.random()).collect())
        .collect()
}

fn mutate_point(rng: &mut ThreadRng, p: &Vec<bool>, r: usize, mutate_dup: bool) -> Vec<bool> {
    let indices = random_indices(rng, p.len(), r, mutate_dup);
    let mut mutated = p.clone();
    for i in indices {
        mutated[i] = !mutated[i];
    }
    mutated
}

struct HashFn {
    indices: Vec<usize>,
    table: HashMap<Vec<bool>, Vec<Vec<bool>>>,
}

impl HashFn {
    fn new(rng: &mut ThreadRng, d: usize, k: usize) -> Self {
        Self {
            indices: random_indices(rng, d, k, false),
            table: HashMap::new(),
        }
    }

    pub fn hash(&self, p: &Vec<bool>) -> Vec<bool> {
        self.indices.iter().map(|&i| p[i]).collect()
    }

    pub fn insert(&mut self, p: Vec<bool>) {
        let key = self.hash(&p);
        self.table.entry(key).or_default().push(p);
    }

    pub fn get(&self, q: &Vec<bool>) -> Vec<Vec<bool>> {
        let key = self.hash(q);
        self.table.get(&key).cloned().unwrap_or_default()
    }
}

fn generate_hash_fns(rng: &mut ThreadRng, l: usize, d: usize, k: usize) -> Vec<HashFn> {
    (0..l).map(|_| HashFn::new(rng, d, k)).collect()
}

fn generate_queries(
    rng: &mut ThreadRng,
    points: &Vec<Vec<bool>>,
    m: usize,
    r: usize,
    mutate_dup: bool,
) -> Vec<Vec<bool>> {
    (0..m)
        .map(|_| {
            let idx = rng.random_range(0..points.len());
            mutate_point(rng, &points[idx], r, mutate_dup)
        })
        .collect()
}

fn lsh_main(
    rng: &mut ThreadRng,
    d: usize,
    n: usize,
    m: usize,
    k: usize,
    l: usize,
    r: usize,
    mutate_dup: bool,
) -> ANNResult {
    let points = generate_points(rng, n, d);
    let queries = generate_queries(rng, &points, m, r, mutate_dup);

    // `g` and the corresponding tables
    let mut tables = generate_hash_fns(rng, l, d, k);
    for table in tables.iter_mut() {
        for p in points.iter() {
            table.insert(p.clone());
        }
    }

    // answer queries
    let correct_nn: usize = queries
        .par_iter()
        .map(|q| {
            let mut answer_d: Option<usize> = None;
            let mut seen = 0;
            'scan: for table in tables.iter() {
                for p in table.get(q) {
                    let d = hamming_distance(&p, q);
                    if answer_d.map_or(true, |ad| d < ad) {
                        answer_d = Some(d);
                    }
                    seen += 1;
                    if seen >= 3 * l {
                        break 'scan;
                    }
                }
            }

            let Some(answer_d) = answer_d else { return 0 };
            let true_d = points.iter().map(|p| hamming_distance(p, q)).min().unwrap();
            assert!(true_d <= answer_d);
            (true_d == answer_d) as usize
        })
        .sum();

    ANNResult {
        correct: correct_nn as f64 / m as f64,
    }
}

fn main() {
    let args = Args::parse();

    if args.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.threads)
            .build_global()
            .unwrap();
    }

    let mut rng = rand::rng();
    let result = lsh_main(
        &mut rng,
        args.d,
        args.n,
        args.m,
        args.k,
        args.l,
        args.r,
        args.mutate_dup,
    );
    println!("Correct rate: {:.2}%", result.correct * 100.0);
}
