use ahash::random_state::RandomState;
use criterion::*;
use criterion::measurement::Measurement;
use gumbel_estimation::GumbelEstimator;
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
use std::hash::Hash;

#[macro_export]
macro_rules! perform {
    ("HyperLogLog", $g:expr, $m:expr, $card:expr, $data:expr) => {
        estimate_hll(&mut ($g), $m, $card, &($data));
    };
    ("Gumbel", $g:expr, $m:expr, $card:expr, $data:expr) => {
        estimate_gumbel(&mut ($g), $m, $card, &($data));
    };
}

pub fn estimate_hll<T, M>(g: &mut BenchmarkGroup<M>, prec: u8, card: usize, data: &[T])
where
    T: Hash,
    M: Measurement,
{
    let mut estimates = Vec::new();
    g.bench_with_input(BenchmarkId::new("HyperLogLog", format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = HyperLogLogPF::<T, _>::new(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.insert(d);
        }
        let estimate = estimator.count();
        println!("{estimate}");
        estimates.push(estimate);
    }));
    println!("{}", estimates.iter().sum::<f64>() / estimates.len() as f64);
}

pub fn estimate_gumbel<T, M>(g: &mut BenchmarkGroup<M>, prec: u8, card: usize, data: &[T])
where
    T: Hash,
    M: Measurement,
{
    let mut estimates = Vec::new();
    g.bench_with_input(BenchmarkId::new("Gumbel", format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = GumbelEstimator::<_>::with_precision(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.add(d);
        }
        let estimate = estimator.count();
        println!("{estimate}");
        estimates.push(estimate);
    }));
    println!("{}", estimates.iter().sum::<f64>() / estimates.len() as f64);
}
/*
 *
 *#[macro_export]
 *macro_rules! estimate_hll {
 *    ($g:expr, $t:ty, $prec:expr, $card:expr, $data:expr) => {
 *        $g.bench_with_input(BenchmarkId::new("HyperLogLog", format!("{}/{}/{}", 1 << $prec, $card, $data.len())), &($data), |b, data| b.iter(|| {
 *            let mut estimator = HyperLogLogPF::<$t, _>::new($prec, RandomState::new()).unwrap();
 *            for d in data {
 *                estimator.insert(d);
 *            }
 *            let estimate = estimator.count();
 *        }));
 *    };
 *}
 *
 *#[macro_export]
 *macro_rules! estimate_gumbel {
 *    ($g:expr, $regs:expr, $card:expr, $data:expr) => {
 *        $g.bench_with_input(BenchmarkId::new("Gumbel", format!("{}/{}/{}", $regs, $card, $data.len())), &($data), |b, data| b.iter(|| {
 *            let mut estimator = GumbelEstimator::<$regs>::new();
 *            for d in data {
 *                estimator.add(d);
 *            }
 *            let estimate = estimator.estimate();
 *        }));
 *    };
 *}
 */
