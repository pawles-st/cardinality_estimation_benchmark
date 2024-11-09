use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;

struct Setup {
    out: File,
    card: usize,
    size: usize,
}

fn print_help() {
    println!("Usage: cargo run <output_file> <card> <size>\n");
    println!("Arguments:");
    println!("- output_file - the file data will be saved to");
    println!("- card - the cardinality of the underlying dataset");
    println!("- size - the total size of the dataset");
    println!("Example:");
    println!("cargo run data_1000_100000.txt 1000 100000");
}

fn parse_args() -> Result<Setup, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        print_help();
        return Err("Incorrect number of arguments provided".into());
    }

    let out = File::create(&args[1])?;
    let card = args[2].parse::<usize>()?;
    let size = args[3].parse::<usize>()?;

    if card > size {
        return Err("dataset size has to be at least the size of its cardinality".into());
    }

    return Ok(Setup{out, card, size});
}

fn gen_data(s: &mut Setup) -> io::Result<()> {
    let mut rng = thread_rng();
    let unif_elem = Uniform::new(0, u64::MAX);
    let unif_index = Uniform::new(0, s.card);

    let mut universe = HashSet::new();
    while universe.len() < s.card {
        let elem = rng.sample(unif_elem);
        universe.insert(elem);
    }

    universe.iter().try_for_each(|elem| {
        writeln!(s.out, "{}", elem)
    })?;

    let universe_vec: Vec<u64> = universe.into_iter().collect();
    let no_duplicates = s.size - s.card;
    (0..no_duplicates).try_for_each(|_| {
        let index = rng.sample(unif_index);
        let elem = universe_vec[index];
        writeln!(s.out, "{}", elem)
    })?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut setup = parse_args()?;
    gen_data(&mut setup)?;

    Ok(())
}
