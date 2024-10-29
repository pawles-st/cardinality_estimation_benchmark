use gumbel_estimation::*;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

fn main() {
    let universe: Vec<u64> = (0..10_000).collect();
    let cardinality = universe.len();
    
    let mut rng = thread_rng();

    let dataset_size = 1_000_000;
    let uniform = Uniform::new(0, cardinality);
    let dataset = (0..dataset_size).map(|_| {
        let i = rng.sample(uniform);
        return universe[i];
    });

    let mut gumbel_estimator = GumbelEstimator::<400>::new();

    for elem in dataset {
        gumbel_estimator.add(&elem);
    }
    println!("estimated cardinality: {}", gumbel_estimator.estimate().round());
}
