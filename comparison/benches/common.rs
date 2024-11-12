use ahash::random_state::RandomState;
use criterion::*;
use criterion::measurement::Measurement;
use gumbel_estimation::GumbelEstimator;
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
use std::hash::Hash;

pub fn bench_hll<T, M>(g: &mut BenchmarkGroup<M>, prec: u8, card: usize, data: &[T])
where
    T: Hash,
    M: Measurement,
{
    g.bench_with_input(BenchmarkId::new("HyperLogLog", format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = HyperLogLogPF::<T, _>::new(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.insert(d);
        }
        let _estimate = estimator.count();
    }));
}

pub fn bench_gumbel<T, M>(g: &mut BenchmarkGroup<M>, prec: u8, card: usize, data: &[T])
where
    T: Hash,
    M: Measurement,
{
    g.bench_with_input(BenchmarkId::new("Gumbel", format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = GumbelEstimator::<_>::with_precision(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.add(d);
        }
        let _estimate = estimator.count();
    }));
}
