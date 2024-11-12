use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};

pub mod constants;

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

