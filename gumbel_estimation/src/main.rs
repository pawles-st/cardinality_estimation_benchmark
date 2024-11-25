use gumbel_estimation::{GHLL, GHLLPlus};
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
    let data = load_data(10000, 1_000_000)?;

    let mut ghll = GHLL::<_>::with_precision(10, builder.clone()).unwrap();
    for d in data.iter() {
        ghll.add(&d);
    }
    println!("GHLL (geo): {}", ghll.count_geo());
    println!("GHLL (har): {}", ghll.count_har());

    let mut ghllp = GHLLPlus::<_>::with_precision(10, builder.clone()).unwrap();
    for d in data.iter() {
        ghllp.add(&d);
    }
    println!("GHLL Plus: {}", ghllp.count());

    Ok(())
}
