#!/bin/bash

cardinalities=(1000 10000 100000 1000000)
size_multiplies=(1 10 100 1000 10000)

for card in "${cardinalities[@]}"; do
	for mult in "${size_multiplies[@]}"; do
		size=$((card * mult))
		out="data/data_${card}_${size}.txt"
		echo "generating card=${card} size=${size} to ${out}"
		cargo run --release -q --manifest-path="gen_data/Cargo.toml" "${out}" ${card} ${size}
	done
done
