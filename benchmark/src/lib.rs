use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};

pub const CARDINALITIES: [usize; 4] = [1_000, 10_000, 100_000, 1_000_000];
pub const DATA_SIZE_MULTIPLIES: [usize; 5] = [1, 10, 100, 1_000, 10_000];
pub const ITERATIONS: usize = 100;

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

