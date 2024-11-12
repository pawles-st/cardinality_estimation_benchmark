use ahash::random_state::RandomState;
use gumbel_estimation::GumbelEstimator;
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
//use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::{Write};
use std::hash::Hash;

use comparison::constants::ITERATIONS;

pub fn gather_hll<T: Hash>(prec: u8, card: usize, data: &[T]) {
    let path = format!("../results/HyperLogLog_{}_{}_{}.txt", prec, card, data.len());
    let mut out = File::create(path).unwrap_or_else(|e| panic!("{}", e));
    
    (0..ITERATIONS).for_each(|_| {
        let mut estimator = HyperLogLogPF::<T, _>::new(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.insert(d);
        }
        let estimate = estimator.count();
        writeln!(out, "{}", estimate).unwrap_or_else(|e| panic!("{}", e));
    });
}

pub fn gather_gumbel<T: Hash>(prec: u8, card: usize, data: &[T]) {
    let path = format!("../results/Gumbel_{}_{}_{}.txt", prec, card, data.len());
    let mut out = File::create(path).unwrap_or_else(|e| panic!("{}", e));
    
    (0..ITERATIONS).for_each(|_| {
        let mut estimator = GumbelEstimator::<_>::with_precision(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.add(d);
        }
        let estimate = estimator.count();
        writeln!(out, "{}", estimate).unwrap_or_else(|e| panic!("{}", e));
    });
}
