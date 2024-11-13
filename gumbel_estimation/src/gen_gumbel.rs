// create a gumbel random value from a bit representation of a [0, 1) float
#[inline(always)]
pub fn from_bits(bits: u32) -> f32 {
    // create the exponent and mantissa bits
    let exponent_bits = 127 << 23;
    let mantissa_bits = bits >> 9;

    // combine the bits
    let bits = exponent_bits | mantissa_bits;

    // create a random [0, 1) float
    let random_unif = f32::from_bits(bits) - 1.0;

    // create a gumbel random variable
    quantile(random_unif)
}

// create a gumbel random value from a [0, 1) float
#[inline(always)]
pub fn quantile(q: f32) -> f32 {
    -f32::ln(-f32::ln(q))
}

// create a gumbel random value from a bit representation of a [0, 1) float,
// but round the result to an integer with the rounding value of `c`
#[inline(always)]
pub fn from_bits_rounded(hash: u32, c: f32) -> u32 {
    let gumbel_value = from_bits(hash);
    shift_round(gumbel_value, c)
}

// create a gumbel random value from a [0, 1) float
// but round the result to an integer with the rounding value of `c`
#[inline(always)]
pub fn quantile_rounded(q: f32, c: f32) -> u32 {
    let gumbel_value = quantile(q);
    shift_round(gumbel_value, c)
}

// perform shift rounding of a value using the rounding value of `c`
#[inline(always)]
pub fn shift_round(value: f32, c: f32) -> u32 {
    f32::floor(value + c) as u32
}