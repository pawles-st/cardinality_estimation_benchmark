use itertools::iproduct;
use std::io::{stdout, Write};
use std::thread;
use std::sync::{Arc, Mutex};

use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES, PRECISIONS};

mod common;

use crate::common::{gather_hll, gather_gumbel};

fn main() {
    println!("Gathering results...");

    // take dataset specifications based on all combinations of
    // (cardinality, data_size) using the constants from constants.rs;
    // datasets of size larger than a billion are ignored
    let no_datasets = CARDINALITIES.len() * DATA_SIZE_MULTIPLIES.len();
    let data_sizes = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES);

    // prepare the handles
    let mut handles = Vec::new();

    // create the counter for experiments done or in progress
    let in_progress_all = Arc::new(Mutex::new(0));

    // gather the results; split the gatherer into threads based on precision
    for prec in PRECISIONS {
        // total number of experiments
        let total_experiments = no_datasets * PRECISIONS.len();

        // clone the data iterators
        let data_sizes_clone = data_sizes.clone();

        // get a shared reference to the counter of completed experiments
        let in_progress = Arc::clone(&in_progress_all);

        let handle = thread::Builder::new()
            .name(format!("Thread prec={}", prec))
            .spawn(move || {
            for (card, mult) in data_sizes_clone {
                {
                    // update the completed datasets counter
                    let mut count = in_progress.lock().unwrap();
                    *count += 1;
                    print!("\rin progress: {}/{}", count, total_experiments);
                    stdout().flush().unwrap();
                }

                // check dataset size; ignore if too large
                if card * mult <= 1_000_000_000 {
                    // gather Hyperloglog results
                    gather_hll(prec, card, card * mult).unwrap();

                    // gather Gumbel results
                    gather_gumbel(prec, card, card * mult).unwrap();
                } else {
                    println!("Dataset is too large - skipping...");
                }
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
