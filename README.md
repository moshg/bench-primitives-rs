# bench-primitives-rs

Benchmarks for primitive types.

## Benchmark

Environment:

* CPU: Intel Core i3-8100 3.60GHz
* RAM: 16GB
* OS: Windows10 1809
* Rust 1.32.0 x86_64-pc-windows-msvc

|name|time|
|---|---|
|bench_add_u8|10 ns/iter (+/- 0)|
|bench_add_u16|27 ns/iter (+/- 2)|
|bench_add_u32|52 ns/iter (+/- 0)|
|bench_add_u64|104 ns/iter (+/- 0)|
|bench_add_u128|289 ns/iter (+/- 4)|

|name|time|
|---|---|
|bench_mul_xor_u8|89 ns/iter (+/- 2)|
|bench_mul_xor_u16|83 ns/iter (+/- 2)|
|bench_mul_xor_u32|84 ns/iter (+/- 2)|
|bench_mul_xor_u64|83 ns/iter (+/- 0)|
|bench_mul_xor_u128|157 ns/iter (+/- 5)|

|name|time|
|---|---|
|bench_rem|484 ns/iter (+/- 7)|
|bench_rem_if|134 ns/iter (+/- 2)|
