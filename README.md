# Space efficient quantile

This repo will implement one (or more) space-efficient algorithm to compute quantiles (like median).

This is mostly an exercise of Rust :)

*IN PROGRESS*

## Benchmark

The results of benchmark implemented by the file [./run_benchmark.sh](run_benchmark.sh) and run in AMD® Ryzen 7 2700x were as follows:

Algorithm | Max error | Num threads | Num elements | Time | Memory
---|---|---|---|---|---|---
Naive | 0% | 1 | 100M | 18s | 1100MB
GK | 1% | 1 | 100M | 23s | 3.0MB
GK | 1% | 8 | 100M | 3.1s | 3.1MB
Modified GK | 1% | 1 | 100M | 9.1s | 2.7MB
Modified GK | 1% | 8 | 100M | 1.3s | 3.1MB
Modified GK | 1% | 1 | 1G | 88s | 2.7MB
Modified GK | 1% | 8 | 1G | 13s | 3.2MB
Modified GK | 0.1% | 8 | 1G | 17s | 3.8MB

Some notes:

1. The naive algorithm simply stores all values in memory, sorts them and then grabs the median term. It has maximum precision however it has linear memory complexity.
2. The GK algorithm was implemented as close to the original [Space-Efficient Online Computation of QuantileSummaries](http://infolab.stanford.edu/~datar/courses/cs361a/papers/quantiles.pdf) article as I could, with a custom modification to allow for parallel execution, since the article does not give any guidance on how to merge different structures.
3. The modified GK algorithm was inspired by the original but has some fundamental changes in the structure and implementation. This is the algorithm exposed at root level by this library.