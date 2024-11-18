use gumbel_estimation::{GumbelEstimator, GumbelEstimatorLazy};
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn load_data(card: usize, size: usize) -> Result<Vec<u64>, io::Error>
{
    let file = File::open(format!("../data/data_{}_{}.txt", card, size))?;
    let reader = BufReader::new(file);

    reader.lines().map(|l| {
        l.and_then(|l| l.trim().parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        )
    }).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let builder = RandomState::new();
    let data = load_data(600_000, 60_000_000)?;

    let mut g = GumbelEstimator::<_>::with_precision(12, builder.clone()).unwrap();
    for d in data {
        g.add(&d);
    }
    println!("{}", g.count_geo());
    println!("{}", g.count_har());

    //let mut gl = GumbelEstimatorLazy::<_>::with_precision(8, builder.clone()).unwrap();
    //for d in data {
        //gl.add(&d);
    //}
    //println!("{}", gl.count_geo());

    Ok(())
}
