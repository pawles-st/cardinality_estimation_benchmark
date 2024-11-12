// cardinalities of the underlying multisets
pub const CARDINALITIES: [usize; 4] = [1_000, 10_000, 100_000, 1_000_000];

// dataset size multiplies; the size of the dataset
// is calculated as `cardinality * data_size_multiply`
pub const DATA_SIZE_MULTIPLIES: [usize; 3] = [1, 100, 10_000];

// precisions to use for the HyperLogLog and Gumbel estimators;
// the number of registers used is equal to `2^precision`
pub const PRECISIONS: [u8; 4] = [4, 8, 12, 16];

// the number of iterations per single dataset
pub const ITERATIONS: usize = 100;
