use ahash::{AHasher, RandomState};
use std::hash::{Hash, Hasher, BuildHasher};
use std::iter::zip;

const NEG_GAMMA: f64 = -0.577215664901532860606512090082402431_f64;

pub struct GumbelEstimator<const K: usize> {
    observables: [f64; K],
    hashers: [AHasher; K],
}

impl<const K: usize> GumbelEstimator<K> {

    pub fn new() -> Self {
        let hash_functions = (0..K).map(|_| {
            RandomState::new().build_hasher()
        }).collect::<Vec<_>>().try_into().unwrap();
        GumbelEstimator{ observables: [f64::NEG_INFINITY; K], hashers: hash_functions }
    }

    pub fn add<T: Hash>(&mut self, value: &T) {
        let observables_iter = self.observables.iter_mut();
        let hashers_iter = self.hashers.iter();
        for (observable, hasher) in zip(observables_iter, hashers_iter) {
            let gumbel_value = GumbelEstimator::<K>::gen_gumbel(value, hasher);
            *observable = f64::max(*observable, gumbel_value);
        }
    }

    pub fn estimate(&self) -> f64 {
        let observables_mean = (self.observables.iter().sum::<f64>()) / (self.observables.len() as f64);
        return f64::exp(NEG_GAMMA + observables_mean);
    }

    fn gen_gumbel<T: Hash>(value: &T, hasher: &AHasher) -> f64 {
        let mut state = hasher.clone();
        value.hash(&mut state);
        let hash = state.finish();

        let exponent = 1023;
        let mantissa = hash & ((1u64 << 52) - 1);

        let f64_bits = (exponent << 52) | mantissa;
        let random_unif = f64::from_bits(f64_bits) - 1.0;

        return -f64::ln(-f64::ln(random_unif));
    }
}

