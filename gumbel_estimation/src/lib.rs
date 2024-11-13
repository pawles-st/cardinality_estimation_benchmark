use std::hash::{Hash, BuildHasher};
use rand::{Rng, thread_rng};
use rand::distributions::{Uniform};

const NEG_GAMMA: f64 = -0.577_215_664_901_532_9_f64;

mod registers;
mod gen_gumbel;

use crate::registers::Registers;

#[derive(Debug)]
pub enum GumbelError {
    InvalidPrecision,
}

/// A cardinality estimator using the Gumbel distribution
pub struct GumbelEstimator<B: BuildHasher> {
    builder: B,
    precision: u8,
    no_registers: usize,
    registers: Registers,
    //registers: Vec<u32>,
    register_rounds: Vec<f32>,
}

impl<B: BuildHasher> GumbelEstimator<B> {

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

        // choose the randomised rounding values for each register
        let register_rounds: Vec<f32> = (0..no_registers).map(|_| {
            rng.sample(unif)
        }).collect();

        // initialise the registers to random gumbel values
        let mut registers = Registers::new(no_registers);
        for (i, &c) in register_rounds.iter().enumerate() {
            let q = rng.sample(unif);
            let gumbel_value = gen_gumbel::quantile_rounded(q, c);
            registers.set(i, gumbel_value);
        }
        /*
         *let registers = (0..no_registers).map(|i| {
         *    let q = rng.sample(unif);
         *    let gumbel_value = Self::gumbel_quantile(q);
         *    let c = register_rounds[i];
         *    let rounded = Self::shift_round(gumbel_value, c);
         *    u32::min(rounded, (1 << 5) - 1)
         *}).collect();
         */

        // create the estimator object
        Ok(Self {
            builder,
            precision,
            no_registers,
            registers,
            register_rounds,
        })
    }

    pub fn add<H: Hash + ?Sized>(&mut self, value: &H) {
        // obtain the value's hash
        let mut hash = self.builder.hash_one(value) as u32; // TODO: u64
        
        // choose a register based on the first `precision` bits
        let index: usize = (hash >> (32 - self.precision)) as usize;

        // discard the above bits from the hash
        hash <<= self.precision;

        // create a gumbel random variable
        let gumbel_value = gen_gumbel::from_bits_rounded(
            hash,
            self.register_rounds[index]
        );

        // update the register to the max of the gumbel random variables
        //self.registers[index] = f32::max(self.registers[index], gumbel_value);
        //self.registers[index] = u32::max(self.registers[index], u32::min(gumbel_value, (1 << 5) - 1));
        self.registers.set_greater(index, gumbel_value);
    }

    pub fn count(&self) -> f64 {
        // apply the second half of shift rounding
        // and calculate the mean of the registers
        let registers_sum = self.registers.iter()
            .enumerate()
            //.map(|(i, &val)| val as f64 - self.register_rounds[i] as f64)
            .map(|(i, val)| val as f64 - self.register_rounds[i] as f64)
            .sum::<f64>();
        let registers_mean = registers_sum / (self.no_registers as f64);
        
        // return the cardinality estimate
        self.no_registers as f64 * f64::exp(NEG_GAMMA + 0.5 + registers_mean)
        //self.no_registers as f64 * f64::exp(NEG_GAMMA + registers_mean)
    }
}

/// A cardinality estimator using the Gumbel distribution
pub struct GumbelEstimatorLazy<B: BuildHasher> {
    builder: B,
    precision: u8,
    no_registers: usize,
    registers: Registers,
    //registers: Vec<u32>,
}

impl<B: BuildHasher> GumbelEstimatorLazy<B> {

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
        let unif = Uniform::new(0, u32::MAX);

        // initialise the registers to random 
        let mut registers = Registers::new(no_registers);
        for i in 0..no_registers {
            let q = rng.sample(unif);
            registers.set(i, q);
        }
        /*
         *let registers = (0..no_registers).map(|i| {
         *    let q = rng.sample(unif);
         *    let gumbel_value = Self::gumbel_quantile(q);
         *    let c = register_rounds[i];
         *    let rounded = Self::shift_round(gumbel_value, c);
         *    u32::min(rounded, (1 << 5) - 1)
         *}).collect();
         */

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

        // leave only the next 5 bits of the hash
        hash >>= 27;

        // update the register to the max of the gumbel random variables
        //self.registers[index] = f32::max(self.registers[index], gumbel_value);
        //self.registers[index] = u32::max(self.registers[index], u32::min(gumbel_value, (1 << 5) - 1));
        self.registers.set_greater(index, hash);
    }

    pub fn count(&self) -> f64 {
        // apply the second half of shift rounding
        // and calculate the mean of the registers
        let registers_sum = self.registers.iter()
            //.map(|(i, &val)| val as f64 - self.register_rounds[i] as f64)
            .map(|val| gen_gumbel::from_bits(val << 27) as f64)
            .sum::<f64>();
        let registers_mean = registers_sum / (self.no_registers as f64);
        
        // return the cardinality estimate
        self.no_registers as f64 * f64::exp(NEG_GAMMA + 0.5 + registers_mean)
        //self.no_registers as f64 * f64::exp(NEG_GAMMA + registers_mean)
    }
}
