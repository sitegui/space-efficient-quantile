#!/bin/bash -e

# Build
cargo build --release --bin benchmark

export TIME='Elapsed (wall clock) time: %E\nMaximum resident set size (kbytes): %M'

# Run with 100M elements
echo "**Naive with 100M elements**"
ALGORITHM=NAIVE VALUES=100000000 THREADS=1 \
    command time ./target/release/benchmark
echo "**Original single-thread GK algorithm with 100M elements and 1% max error**"
ALGORITHM=GK VALUES=100000000 THREADS=1 EPSILON=0.01 \
    command time ./target/release/benchmark
echo "**Original multi-thread GK algorithm with 100M elements and 1% max error**"
ALGORITHM=GK VALUES=100000000 THREADS=8 EPSILON=0.01 \
    command time ./target/release/benchmark
echo "**Modified single-thread GK algorithm with 100M elements and 1% max error**"
ALGORITHM=MODIFIED_GK VALUES=100000000 THREADS=1 EPSILON=0.01 \
    command time ./target/release/benchmark
echo "**Modified multi-thread GK algorithm with 100M elements and 1% max error**"
ALGORITHM=MODIFIED_GK VALUES=100000000 THREADS=8 EPSILON=0.01 \
    command time ./target/release/benchmark

# Run with 1G elements
echo "**Modified single-thread GK algorithm with 1G elements and 1% max error**"
ALGORITHM=MODIFIED_GK VALUES=1000000000 THREADS=1 EPSILON=0.01 \
    command time ./target/release/benchmark
echo "**Modified multi-thread GK algorithm with 1G elements and 1% max error**"
ALGORITHM=MODIFIED_GK VALUES=1000000000 THREADS=8 EPSILON=0.01 \
    command time ./target/release/benchmark
echo "**Modified multi-thread GK algorithm with 1G elements and 0.1% max error**"
ALGORITHM=MODIFIED_GK VALUES=1000000000 THREADS=8 EPSILON=0.001 \
    command time ./target/release/benchmark