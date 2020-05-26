# bench-primitives-rs

Benchmarks for primitive types.

## Benchmark

Environment:

* CPU: Intel Core i3-8100 3.60GHz
* RAM: 16GB
* OS: Windows10 1809
<!-- * Rust 1.32.0 x86_64-pc-windows-msvc -->

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

Environment:

* CPU: Intel Celeron 1005M 1.90GHz
* RAM: 4.00GB
* OS: Windows 10 Pro 1909
* nightly-x86_64-pc-windows-msvc 2020/05/26

|name|time|
|---|---|
|bench_add_u8|22 ns/iter (+/- 5)|
|bench_add_u16|64 ns/iter (+/- 3)|
|bench_add_u32|127 ns/iter (+/- 4)|
|bench_add_u64|266 ns/iter (+/- 24)|
|bench_add_u128|1053 ns/iter (+/- 103)|

|name|time|
|---|---|
|bench_mul_xor_u8|207 ns/iter (+/- 27)|
|bench_mul_xor_u16|168 ns/iter (+/- 6)|
|bench_mul_xor_u32|174 ns/iter (+/- 6)|
|bench_mul_xor_u64|168 ns/iter (+/- 10)|
|bench_mul_xor_u128|325 ns/iter (+/- 12)|

|name|time|
|---|---|
|bench_rem|926 ns/iter (+/- 321)|
|bench_rem_if|274 ns/iter (+/- 12)|
