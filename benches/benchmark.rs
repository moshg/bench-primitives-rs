#[macro_use]
extern crate criterion;
extern crate rand;

use std::ops::{Add, BitXor, Mul};
use criterion::{Criterion, black_box};
use rand::{random, distributions::{Standard, Distribution}, thread_rng, Rng};

fn bench_add<T>(id: &str, c: &mut Criterion) where T: Copy + Default + Add<Output=T>, Standard: Distribution<T> {
    let mut nums = [T::default(); 1000];
    for i in nums.iter_mut() {
        *i = random();
    }

    c.bench_function(id, |b| b.iter(|| {
        black_box(nums).iter().fold(T::default(), |i, &j| i + j);
    }));
}

fn bench_adds(c: &mut Criterion) {
    bench_add::<u8>("u8 add", c);
    bench_add::<u16>("u16 add", c);
    bench_add::<u32>("u32 add", c);
    bench_add::<u64>("u64 add", c);
    bench_add::<u128>("u128 add", c);
}

fn bench_u16_add(c: &mut Criterion) {
    bench_add::<u16>("u16 add", c);
}

fn bench_u32_add(c: &mut Criterion) {
    bench_add::<u32>("u32 add", c);
}

fn bench_u64_add(c: &mut Criterion) {
    bench_add::<u64>("u64 add", c);
}

fn bench_u128_add(c: &mut Criterion) {
    bench_add::<u128>("u128 add", c);
}

fn bench_mul_xor<T>(id: &str, c: &mut Criterion)
    where T: Copy + Default + Mul<Output=T> + BitXor<Output=T>, Standard: Distribution<T>
{
    let mut nums = [T::default(); 100];
    for i in nums.iter_mut() {
        *i = random();
    }

    c.bench_function(id, |b| b.iter(|| {
        black_box(nums).iter().fold(T::default(), |i, &j| (i * j) ^ j);
    }));
}

fn bench_mul_xors(c: &mut Criterion) {
    bench_mul_xor::<u8>("u8 mul xor", c);
    bench_mul_xor::<u16>("u16 mul xor", c);
    bench_mul_xor::<u32>("u32 mul xor", c);
    bench_mul_xor::<u64>("u64 mul xor", c);
    bench_mul_xor::<u128>("u128 mul xor", c);
}

fn bench_rem(c: &mut Criterion) {
    let mut nums = [0i8; 100];
    for i in nums.iter_mut() {
        *i = thread_rng().gen_range(-30, 30);
    }

    c.bench_function("rem", |b| b.iter(|| {
        black_box(nums).iter().fold(0, |i, &j| (i + j) % 30);
    }));
}

fn bench_branch_rem(c: &mut Criterion) {
    let mut nums = [0i8; 100];
    for i in nums.iter_mut() {
        *i = thread_rng().gen_range(-30, 30);
    }

    c.bench_function("branch rem", |b| b.iter(|| {
        black_box(nums).iter().fold(0, |i, &j| {
            let sum = i + j;
            if sum < 30 {
                i
            } else {
                i - 30
            }
        });
    }));
}

criterion_group!(benches, bench_adds, bench_mul_xors, bench_rem, bench_branch_rem);
criterion_main!(benches);
