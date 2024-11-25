// create a const array from a start value and step
const fn array_from_range<const K: usize>(begin: usize, step: usize) -> [usize; K] {
    // create the array
    let mut result = [0; K];

    // Fill the array with values
    let mut curr = begin;
    let mut index = 0;
    while index < K {
        result[index] = curr;
        curr += step;
        index += 1;
    }

    result   
}

// cardinalities of the underlying multisets
pub const CARDINALITIES: [usize; 80] = array_from_range(10_000, 10_000);

// dataset size multiplies; the size of the dataset
// is calculated as `cardinality * data_size_multiply`
pub const DATA_SIZE_MULTIPLIES: [usize; 1] = [100];

// precisions to use for the HyperLogLog and Gumbel estimators;
// the number of registers used is equal to `2^precision`
pub const PRECISIONS: [u8; 4] = [4, 8, 12, 16];

// the number of iterations per single dataset
pub const ITERATIONS: usize = 100;
