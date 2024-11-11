use std::hash::{Hash, BuildHasher};
use rand::{Rng, thread_rng};
use rand::distributions::{Uniform};

const NEG_GAMMA: f64 = -0.577215664901532860606512090082402431_f64;

#[derive(Debug)]
pub enum GumbelError {
    InvalidPrecision,
}

/// A cardinality estimator using the Gumbel distribution
pub struct GumbelEstimator<B: BuildHasher> {
    builder: B,
    precision: u8,
    no_registers: usize,
    registers: Vec<f32>,
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
    /// formula `no_registers = 2^precision`; the accepted values lie in the range {4, 5, ..., 16}
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
        let registers = (0..no_registers).map(|_| {
            -f32::ln(-f32::ln(rng.sample(unif)))
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
        // obtain the value's hash
        let mut hash = self.builder.hash_one(value) as u32; // TODO: u64
        
        // choose a register based on the first `precision` bits
        let index: usize = (hash >> (32 - self.precision)) as usize;

        // discard the above bits from the hash
        hash <<= self.precision;

        // create a gumbel random variable
        let gumbel_value = Self::gen_gumbel(hash);

        // update the register to the max of the gumbel random variables
        self.registers[index] = f32::max(self.registers[index], gumbel_value);
        //self.registers.set_greater(register_index, gumbel_value);
    }

    pub fn count(&self) -> f64 {
        // calculate the mean of the registers
        let registers_mean = self.registers.iter().map(|&val| val as f64).sum::<f64>() / (self.no_registers as f64);
        
        // return the cardinality estimate
        self.no_registers as f64 * f64::exp(NEG_GAMMA + registers_mean)
    }

    #[inline(always)]
    fn gen_gumbel(hash: u32) -> f32 {
        // create the exponent and mantissa bits
        let exponent_bits = 127 << 23;
        let mantissa_bits = hash >> 9;

        // combine the bits
        let bits = exponent_bits | mantissa_bits;

        // create a random [0, 1) float
        let random_unif = f32::from_bits(bits) - 1.0;

        // create a gumbel random variable
        -f32::ln(-f32::ln(random_unif))
    }
}
