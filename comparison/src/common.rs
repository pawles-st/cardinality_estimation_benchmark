use ahash::random_state::RandomState;
use gumbel_estimation::GumbelEstimator;
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
//use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use comparison::constants::ITERATIONS;

pub fn gather_hll(prec: u8, card: usize, size: usize) -> Result<(), io::Error> {

    // prepare the output
    let outpath = format!("../results/HyperLogLog_{}_{}_{}.txt", prec, card, size);
    let mut out = File::create(outpath)?;
    
    // gather the data
    for _ in 0..ITERATIONS {
        // prepare the input data
        let input = File::open(format!("../data/data_{}_{}.txt", card, size))?;
        let reader = BufReader::new(input);

        // create the estimator
        let mut estimator = HyperLogLogPF::<u64, _>::new(prec, RandomState::new()).unwrap();

        // feed the data to the estimator
        for line in reader.lines() {
            let value = line.and_then(|l| l.trim().parse::<u64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            )?;
            estimator.insert(&value);
        };

        // acquire the cardinality estimate
        let estimate = estimator.count();

        // write the result to the out file
        writeln!(out, "{}", estimate).unwrap_or_else(|e| panic!("{}", e));
    };

    Ok(())
}

pub fn gather_gumbel(prec: u8, card: usize, size: usize) -> Result<(), io::Error> {
    // prepare the output
    let outpath = format!("../results/Gumbel_{}_{}_{}.txt", prec, card, size);
    let mut out = File::create(outpath)?;
    
    // gather the data
    for _ in 0..ITERATIONS {
        // prepare the input data
        let input = File::open(format!("../data/data_{}_{}.txt", card, size))?;
        let reader = BufReader::new(input);

        // create the estimator
        let mut estimator = GumbelEstimator::<_>::with_precision(prec, RandomState::new()).unwrap();

        // feed the data to the estimator
        for line in reader.lines() {
            let value = line.and_then(|l| l.trim().parse::<u64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            )?;
            estimator.add(&value);
        };

        // acquire the cardinality estimate
        let estimate = estimator.count();

        // write the result to the out file
        writeln!(out, "{}", estimate).unwrap_or_else(|e| panic!("{}", e));
    };

    Ok(())
}
