use ahash::random_state::RandomState;
use gumbel_estimation::GumbelEstimator;
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
//use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::{stdout, Write};
use std::hash::Hash;

#[macro_export]
macro_rules! perform {
    ("HyperLogLog", $m:expr, $card:expr, $data:expr) => {
        estimate_hll($m, $card, &($data), benchmark::ITERATIONS);
    };
    ("Gumbel", $m:expr, $card:expr, $data:expr) => {
        estimate_gumbel($m, $card, &($data), benchmark::ITERATIONS);
    };
    ("HyperLogLog", $m:expr, $card:expr, $data:expr, $iters:expr) => {
        estimate_hll($m, $card, &($data), $iter);
    };
    ("Gumbel", $m:expr, $card:expr, $data:expr, $iters:expr) => {
        estimate_gumbel($m, $card, &($data), $iter);
    };
}

pub fn estimate_hll<T: Hash>(prec: u8, card: usize, data: &[T], iters: usize) {
    let path = format!("target/accuracy/HyperLogLog_{}_{}_{}.txt", prec, card, data.len());
    let mut out = File::create(path).unwrap_or_else(|e| panic!("{}", e));
    
    println!("Gathering results for HyperLogLog/{}/{}/{}", prec, card, data.len());

    (1..=iters).for_each(|i| {
        print!("\r{}/{}", i, iters);
        stdout().flush().unwrap();

        let mut estimator = HyperLogLogPF::<T, _>::new(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.insert(d);
        }
        let estimate = estimator.count();
        writeln!(out, "{}", estimate).unwrap_or_else(|e| panic!("{}", e));
    });
    println!();
}

pub fn estimate_gumbel<T: Hash>(prec: u8, card: usize, data: &[T], iters: usize) {
    let path = format!("target/accuracy/Gumbel_{}_{}_{}.txt", prec, card, data.len());
    let mut out = File::create(path).unwrap_or_else(|e| panic!("{}", e));
    
    println!("Gathering results for Gumbel/{}/{}/{}", prec, card, data.len());

    (1..=iters).for_each(|i| {
        print!("\r{}/{}", i, iters);
        stdout().flush().unwrap();

        let mut estimator = GumbelEstimator::<_>::with_precision(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.add(d);
        }
        let estimate = estimator.count();
        writeln!(out, "{}", estimate).unwrap_or_else(|e| panic!("{}", e));
    });
    println!();
}
