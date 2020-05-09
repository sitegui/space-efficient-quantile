use crate::quantile_generator::{OrderedF64, RandomGenerator};
use std::env;
use std::thread;

/// Calculate the median of a big sequence of values, used for benchmarking
/// Takes in the environment values:
/// - ALGORITHM: string, one of 'NAIVE', 'GK', 'MODIFIED_GK'
/// - THREADS: uint
/// - VALUES: uint
/// - EPSILON: float
fn main() {
    let values: u64 = env::var("VALUES").unwrap().parse().unwrap();
    let threads: u64 = env::var("THREADS").unwrap().parse().unwrap();
    match env::var("ALGORITHM").unwrap().as_str() {
        "NAIVE" => run_algorithm::<naive::Algorithm>(values, threads),
        "GK" => run_algorithm::<gk::Algorithm>(values, threads),
        "MODIFIED_GK" => run_algorithm::<modified_gk::Algorithm>(values, threads),
        _ => panic!("Invalid choice of algorithm"),
    }
}

trait Summary: Send {
    fn new() -> Self;
    fn insert_one(&mut self, value: OrderedF64);
    fn merge(&mut self, other: Self);
    fn query_with_error(&mut self, quantile: f64) -> Option<(&OrderedF64, f64)>;
}

fn run_algorithm<S: Summary + 'static>(values: u64, threads: u64) {
    let num_values_per_thread = values / threads;

    let thread_handles = (0..threads)
        .map(|seed| {
            thread::spawn(move || {
                let mut summary = S::new();
                for value in RandomGenerator::new(0.5, 17., num_values_per_thread as usize, seed) {
                    summary.insert_one(value);
                }
                summary
            })
        })
        .collect::<Vec<_>>();

    let summaries = thread_handles
        .into_iter()
        .map(|t| t.join().unwrap())
        .collect::<Vec<_>>();

    let mut summary = summaries
        .into_iter()
        .fold(None, |a, b| match a {
            None => Some(b),
            Some(mut a) => {
                a.merge(b);
                Some(a)
            }
        })
        .unwrap();

    println!("{:?}", summary.query_with_error(0.5));
}

mod naive {
    use super::{OrderedF64, Summary};
    use space_efficient_quantile::quantile_to_rank;
    pub struct Algorithm {
        values: Vec<OrderedF64>,
    }
    impl Summary for Algorithm {
        fn new() -> Self {
            Algorithm { values: Vec::new() }
        }
        fn insert_one(&mut self, value: OrderedF64) {
            self.values.push(value);
        }
        fn merge(&mut self, mut other: Self) {
            self.values.append(&mut other.values);
        }
        fn query_with_error(&mut self, quantile: f64) -> Option<(&OrderedF64, f64)> {
            self.values.sort();
            let rank = quantile_to_rank(quantile, self.values.len() as u64);
            Some((&self.values[rank as usize], 0.))
        }
    }
}

mod gk {
    use super::{OrderedF64, Summary};
    use space_efficient_quantile::gk::Summary as InternalSummary;
    use std::env;
    pub struct Algorithm {
        summary: InternalSummary<OrderedF64>,
    }
    impl Summary for Algorithm {
        fn new() -> Self {
            let epsilon: f64 = env::var("EPSILON").unwrap().parse().unwrap();
            Algorithm {
                summary: InternalSummary::new(epsilon),
            }
        }
        fn insert_one(&mut self, value: OrderedF64) {
            self.summary.insert_one(value);
        }
        fn merge(&mut self, other: Self) {
            self.summary.merge(other.summary);
        }
        fn query_with_error(&mut self, quantile: f64) -> Option<(&OrderedF64, f64)> {
            self.summary.query_with_error(quantile)
        }
    }
}

mod modified_gk {
    use super::{OrderedF64, Summary};
    use space_efficient_quantile::modified_gk::Summary as InternalSummary;
    use std::env;
    pub struct Algorithm {
        summary: InternalSummary<OrderedF64>,
    }
    impl Summary for Algorithm {
        fn new() -> Self {
            let epsilon: f64 = env::var("EPSILON").unwrap().parse().unwrap();
            Algorithm {
                summary: InternalSummary::new(epsilon),
            }
        }
        fn insert_one(&mut self, value: OrderedF64) {
            self.summary.insert_one(value);
        }
        fn merge(&mut self, other: Self) {
            self.summary.merge(other.summary);
        }
        fn query_with_error(&mut self, quantile: f64) -> Option<(&OrderedF64, f64)> {
            self.summary.query_with_error(quantile)
        }
    }
}
