use ahash::random_state::RandomState;
use gen_data::generate;
use gumbel_estimation::{GumbelEstimator, GumbelEstimatorLazy};
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
//use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

pub mod constants;

use constants::ITERATIONS;

pub fn create_output(alg: &str, prec: u8, card: usize, size: usize) -> Result<File, io::Error> {
    let outpath = format!("../results/{}_{}_{}_{}.txt", alg, prec, card, size);
    let out = File::create(outpath)?;

    Ok(out)
}

pub fn create_input(card: usize, size: usize) -> Result<BufReader<File>, io::Error> {
    let inpath = format!("../data/data_{}_{}.txt", card, size);
    let input = match File::open(&inpath) {
        Ok(file) => file,
        Err(_) => {
            println!("g");
            let mut file = File::create(&inpath)?;
            generate(&mut file, card, size)?;
            file
        }
    };
    let reader = BufReader::new(input);

    Ok(reader)
}

pub fn gather_hll(prec: u8, card: usize, size: usize) -> Result<(), io::Error> {
    // prepare the output
    let mut out = create_output("HyperLogLog", prec, card, size)?;
    
    // prepare the input data
    let reader = create_input(card, size)?;
    
    // create `ITERATIONS` independent estimators
    let mut estimators: Vec<_> = (0..ITERATIONS).map(|_| HyperLogLogPF::<u64, _>::new(prec, RandomState::new()).unwrap()).collect();

    // analyse the data
    for line in reader.lines() {
        // read the next value
        let value = line.and_then(|l| l.trim().parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        )?;

        // feed the value to each estimator
        for estimator in &mut estimators {
            estimator.insert(&value);
        };
    };

    // acquire the cardinality estimate for each estimator and write the result
    for estimator in &mut estimators {
        let estimate = estimator.count();
        writeln!(out, "{}", estimate)?;
    }

    Ok(())
}

pub fn gather_gumbel(prec: u8, card: usize, size: usize) -> Result<(), io::Error> {
    // prepare the output
    let mut out = create_output("Gumbel", prec, card, size)?;
    
    // prepare the input data
    let reader = create_input(card, size)?;
    
    // create `ITERATIONS` independent estimators
    let mut estimators: Vec<_> = (0..ITERATIONS).map(|_| GumbelEstimator::<_>::with_precision(prec, RandomState::new()).unwrap()).collect();

    // analyse the data
    for line in reader.lines() {
        // read the next value
        let value = line.and_then(|l| l.trim().parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        )?;

        // feed the value to each estimator
        for estimator in &mut estimators {
            estimator.add(&value);
        };
    };

    // acquire the cardinality estimate for each estimator and write the result
    for estimator in &mut estimators {
        let estimate = estimator.count();
        writeln!(out, "{}", estimate)?;
    }

    Ok(())
}

pub fn gather_gumbel_lazy(prec: u8, card: usize, size: usize) -> Result<(), io::Error> {
    // prepare the output
    let mut out = create_output("GumbelLazy", prec, card, size)?;
    
    // prepare the input data
    let reader = create_input(card, size)?;
    
    // create `ITERATIONS` independent estimators
    let mut estimators: Vec<_> = (0..ITERATIONS).map(|_| GumbelEstimatorLazy::<_>::with_precision(prec, RandomState::new()).unwrap()).collect();

    // analyse the data
    for line in reader.lines() {
        // read the next value
        let value = line.and_then(|l| l.trim().parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        )?;

        // feed the value to each estimator
        for estimator in &mut estimators {
            estimator.add(&value);
        };
    };

    // acquire the cardinality estimate for each estimator and write the result
    for estimator in &mut estimators {
        let estimate = estimator.count();
        writeln!(out, "{}", estimate)?;
    }

    Ok(())
}
