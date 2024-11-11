use itertools::iproduct;

use comparison::{CARDINALITIES, DATA_SIZE_MULTIPLIES, load_data};

mod common;

use crate::common::{estimate_hll, estimate_gumbel};

fn main() {

    println!("Gathering results...");

    // take all combinations of (cardinality, data_size) except the largest (that would amount to 200 GB of data)

    let no_datasets = CARDINALITIES.len() * DATA_SIZE_MULTIPLIES.len() - 1;
    let data_sizes = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES).take(no_datasets);

    // gather the results

    for (i, (card, mult)) in data_sizes.enumerate() {

        println!("Analysing dataset {}/{}", i + 1, no_datasets);

        // load data from file
        
        println!("Loading data...");
        let data: Vec<u64> = load_data(card, card * mult)
            .unwrap_or_else(|e| panic!("Error while loading data for card={}, size={}: {}", card, card * mult, e));

        // gather Hyperloglog results

        perform!("HyperLogLog", 4, card, data);

        // gather Gumbel results

        perform!("Gumbel", 4, card, data);
    }
}
