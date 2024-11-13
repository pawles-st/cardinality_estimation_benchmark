use criterion::*;
use itertools::iproduct;

use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES};

mod common;

use crate::common::{bench_hll, bench_gumbel, load_data};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hyperloglog vs Gumbel");
    let data_sizes = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES)
        .take(CARDINALITIES.len() * DATA_SIZE_MULTIPLIES.len() - 1);

    for (card, mult) in data_sizes {

        // read data from file

        let data: Vec<u64> = load_data(card, card * mult)
            .unwrap_or_else(|e| panic!("{}", e));

        // perform Hyperloglog benchmark

        bench_hll(&mut group, 4, card, &data);

        // perform Gumbel benchmark

        bench_gumbel(&mut group, 4, card, &data);
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
