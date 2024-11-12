use itertools::iproduct;
use std::io::{stdout, Write};
use std::thread;
use std::sync::{Arc, Mutex};

use comparison::load_data;
use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES, PRECISIONS};

mod common;

use crate::common::{gather_hll, gather_gumbel};

fn main() {
    println!("Gathering results...");

    // take dataset specifications based on all combinations of
    // (cardinality, data_size) using the constants from constants.rs;
    // datasets of size larger than million are ignored
    let no_datasets = CARDINALITIES.len() * DATA_SIZE_MULTIPLIES.len();
    let data_sizes = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES);

    // prepare the handles
    let mut handles = Vec::new();

    // create the counter of completed experiments
    let completed_all = Arc::new(Mutex::new(0));

    // gather the results; split the gatherer into threads based on precision
    for prec in PRECISIONS {
        // total number of experiments
        let total_experiments = no_datasets * PRECISIONS.len();

        // clone the data iterators
        let data_sizes_clone = data_sizes.clone();

        // get a shared reference to the counter of completed experiments
        let completed = Arc::clone(&completed_all);

        let handle = thread::Builder::new()
            .name(format!("Thread prec={}", prec))
            .spawn(move || {
            for (card, mult) in data_sizes_clone {
                // load data from file
                let data: Vec<u64> = load_data(card, card * mult).unwrap();

                // gather Hyperloglog results
                gather_hll(prec, card, &data);

                // gather Gumbel results
                gather_gumbel(prec, card, &data);

                // update the completed datasets counter
                let mut count = completed.lock().unwrap();
                *count += 1;
                print!("\rcompleted: {}/{}", count, total_experiments);
                stdout().flush().unwrap();
            }
        }).unwrap();

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread panicked: {:?}", e);
        }
    }
}
