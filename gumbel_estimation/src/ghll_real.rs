use std::hash::{Hash, BuildHasher};
use rand::{Rng, thread_rng};
use rand::distributions::{Uniform};

use crate::common::*;
use crate::gen_gumbel;

/// A cardinality estimator using the Gumbel distribution
pub struct GHLLReal<B: BuildHasher> {
    builder: B,
    precision: u8,
    no_registers: usize,
    registers: Vec<f32>,
}

impl<B: BuildHasher> GHLLReal<B> {
    /// Creates a new `GumbelEstimator` object with a custom precision and hash builder
    ///
    /// # Arguments
    ///
    /// - `precision` - corresponds to the number of registers used by this estimator using the 
    ///   formula `no_registers = 2^precision`; the accepted values lie in the range {4, 5, ..., 16}
    /// - `builder` - this is a hash builder that will be used for hashing provided values
    pub fn with_precision(precision: u8, builder: B) -> Result<Self, GumbelError> {
        // check if the provided precision is within the bounds
        if !(MIN_PRECISION..=MAX_PRECISION).contains(&precision) {
            return Err(GumbelError::InvalidPrecision);
        }

        // calculate the number of registers as `2^precision`
        let no_registers = 1 << precision;

        // create a uniform [0, 1) rng
        let mut rng = thread_rng();
        let unif = Uniform::new(0.0, 1.0);

        // initialise the registers to random gumbel values
        let registers: Vec<_> = (0..no_registers).map(|_| {
            let q = rng.sample(unif);
            gen_gumbel::quantile(q)
        }).collect();

        // create the estimator object
        Ok(Self {
            builder,
            precision,
            no_registers,
            registers,
        })
    }

    pub fn add<H: Hash + ?Sized>(&mut self, value: &H) {
        // hash the value and separate the hash into the index and the remainder
        let (index, hash) = hash_value(value, &self.builder, self.precision);

        // create a gumbel random variable
        let gumbel_value = gen_gumbel::from_bits(hash);

        // update the register to the max of the gumbel random variables
        self.registers[index] = f32::max(self.registers[index], gumbel_value);
    }

    pub fn count_geo(&self) -> f64 {
        // apply the second half of shift rounding
        // and calculate the geometric mean of the `exp(register)` terms
        let registers_sum = self.registers.iter()
            .map(|&val| val as f64)
            .sum::<f64>();
        let registers_mean = registers_sum / self.no_registers as f64;
        
        self.no_registers as f64 * f64::exp(NEG_GAMMA + registers_mean)
    }
    
    pub fn count_har(&self) -> f64 {
        // apply the second half of shift rounding
        // and calculate the harmonic mean of the `exp(register)` terms
        let registers_sum = self.registers.iter()
            .map(|&val| f64::exp(-val as f64))
            .sum::<f64>();
        let registers_mean = registers_sum / self.no_registers as f64;
        
        self.no_registers as f64 / registers_mean - 1.0
    }
}