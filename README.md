# benchmark

基本的な型に関するベンチマーク

## ベンチマーク

環境:

* CPU: Intel Core i3-8100 3.60GHz
* RAM: 16GB
* OS: Windows10 1809
* Rust 1.32.0 x86_64-pc-windows-msvc

|name|time|
|---|---|
|bench_u8_add|14 ns/iter (+/- 0)|
|bench_u16_add|27 ns/iter (+/- 3)|
|bench_u32_add|39 ns/iter (+/- 2)|
|bench_u64_add|77 ns/iter (+/- 4)|
|bench_u128_add|287 ns/iter (+/- 8)|

|name|time|
|---|---|
|bench_u8|89 ns/iter (+/- 4)|
|bench_u16|83 ns/iter (+/- 1)|
|bench_u32|84 ns/iter (+/- 5)|
|bench_u64|82 ns/iter (+/- 2)|
|bench_u128|157 ns/iter (+/- 2)|

|name|time|
|---|---|
|bench_rem|457 ns/iter (+/- 7)|
|bench_rem_if|60 ns/iter (+/- 6)|
