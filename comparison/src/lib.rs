use ahash::random_state::RandomState;
use gumbel_estimation::{GumbelEstimator};
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
    let input = File::open(&inpath).map_err(|err| {
        io::Error::new(err.kind(), format!("failed to open file {}", inpath))
    })?;
    let reader = BufReader::new(input);

    Ok(reader)
}

pub fn gather(prec: u8, card: usize, size: usize) -> Result<(), io::Error> {
    // prepare the input data
    let reader = create_input(card, size)?;

    // prepare the output
    let mut hll_out = create_output("HyperLogLog", prec, card, size)?;
    let mut gumbel_geo_out = create_output("GumbelGeo", prec, card, size)?;
    let mut gumbel_har_out = create_output("GumbelHar", prec, card, size)?;
    
    // create `ITERATIONS` independent estimators with a common random state
    let builders: Vec<_> = (0..ITERATIONS).map(|_| RandomState::new()).collect();
    let mut hll_estimators: Vec<_> = (0..ITERATIONS).map(|i| HyperLogLogPF::<u64, _>::new(prec, builders[i].clone()).unwrap()).collect();
    let mut gumbel_estimators: Vec<_> = (0..ITERATIONS).map(|i| GumbelEstimator::<_>::with_precision(prec, builders[i].clone()).unwrap()).collect();

    // analyse the data
    for line in reader.lines() {
        // read the next value
        let value = line.and_then(|l| l.trim().parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        )?;

        // feed the value to each estimator
        for estimator in &mut hll_estimators {
            estimator.insert(&value);
        };
        for estimator in &mut gumbel_estimators {
            estimator.add(&value);
        };
    };

    // acquire the cardinality estimate for each estimator and write the result
    for estimator in &mut hll_estimators {
        let estimate = estimator.count();
        writeln!(hll_out, "{}", estimate)?;
    }
    for estimator in &mut gumbel_estimators {
        let estimate = estimator.count_geo();
        writeln!(gumbel_geo_out, "{}", estimate)?;
        let estimate = estimator.count_har();
        writeln!(gumbel_har_out, "{}", estimate)?;
    }

    Ok(())
}

