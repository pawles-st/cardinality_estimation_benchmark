use std::f64::consts::E;
use std::hash::{Hash, BuildHasher};
use rand::{Rng, thread_rng};
use rand::distributions::{Uniform};

use crate::common::{NEG_GAMMA, GumbelError};
use crate::gen_gumbel;
use crate::registers::Registers;

const THRESHOLD: f32 = 0.731_447_7;

/// A cardinality estimator using the Gumbel distribution
pub struct GHLLS<B: BuildHasher> {
    builder: B,
    precision: u8,
    no_registers: usize,
    registers: Registers,
}

impl<B: BuildHasher> GHLLS<B> {

    /// The minumal accepted precision
    const MIN_PRECISION: u8 = 4;

    /// The maximmal accepted precision
    const MAX_PRECISION: u8 = 16;

    /// Creates a new `GumbelEstimator` object with a custom precision and hash builder
    ///
    /// # Arguments
    ///
    /// - `precision` - corresponds to the number of registers used by this estimator using the 
    ///   formula `no_registers = 2^precision`; the accepted values lie in the range {4, 5, ..., 16}
    /// - `builder` - this is a hash builder that will be used for hashing provided values
    pub fn with_precision(precision: u8, builder: B) -> Result<Self, GumbelError> {
        // check if the provided precision is within the bounds
        if !(Self::MIN_PRECISION..=Self::MAX_PRECISION).contains(&precision) {
            return Err(GumbelError::InvalidPrecision);
        }

        // calculate the number of registers as `2^precision`
        let no_registers = 1 << precision;

        // create a uniform [0, 1) rng
        let mut rng = thread_rng();
        let unif = Uniform::new(0.0, 1.0);

        // initialise the registers to random gumbel values
        let mut registers = Registers::new(no_registers);
        for i in 0..no_registers {
            let q = rng.sample(unif);
            let c = gen_gumbel::mantissa_to_float(builder.hash_one(i) as u32);
            let gumbel_value = gen_gumbel::quantile_rounded(q, c);
            registers.set(i, gumbel_value);
        }

        // create the estimator object
        Ok(Self {
            builder,
            precision,
            no_registers,
            registers,
        })
    }

    pub fn add<H: Hash + ?Sized>(&mut self, value: &H) {
        // obtain the value's hash
        let mut hash = self.builder.hash_one(value) as u32;
        
        // choose a register based on the first `precision` bits
        let index: usize = (hash >> (32 - self.precision)) as usize;

        // discard the above bits from the hash
        hash <<= self.precision;

        // create a gumbel random variable
        let gumbel_value = gen_gumbel::from_bits_rounded(
            hash,
            gen_gumbel::mantissa_to_float(self.builder.hash_one(index) as u32)
        );

        // update the register to the max of the gumbel random variables
        self.registers.set_greater(index, gumbel_value);
    }

    pub fn count(&self) -> f64 {
        // perform a free register hypothesis test for each register
        let no_free = self.registers.iter().enumerate().filter(|(i, val)| *val as f32 - gen_gumbel::mantissa_to_float(self.builder.hash_one(i) as u32) < THRESHOLD).count();
        
        // apply low-range correction
        if no_free != 0 && (no_free as f64) >= self.no_registers as f64 / E  {
            // perform linear counting
            return self.no_registers as f64 * f64::ln(self.no_registers as f64 / no_free as f64);
        }

        // apply the second half of shift rounding
        // and calculate the geometric mean of the `exp(register)` terms
        let registers_sum = self.registers.iter()
            .enumerate()
            .map(|(i, val)| val as f64 - gen_gumbel::mantissa_to_float(self.builder.hash_one(i) as u32) as f64)
            .sum::<f64>();
        let registers_mean = registers_sum / self.no_registers as f64;
        
        self.no_registers as f64 * f64::exp(NEG_GAMMA + 0.5 + registers_mean) - self.no_registers as f64 / 2.0 - 1.5
    }
}
