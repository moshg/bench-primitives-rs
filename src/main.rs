#![feature(test)]

extern crate test;
extern crate rand;

use test::Bencher;
use std::ops::{Add, Mul, BitXor};
use rand::{random, Rng, distributions::{Distribution, Standard}};

fn bench_add<T>(b: &mut Bencher) where T: Copy + Default + Add<Output=T>, Standard: Distribution<T> {
    let mut nums = [T::default(); 1000];
    for i in nums.iter_mut() {
        *i = random();
    }
    b.iter(|| {
        nums.iter().fold(T::default(), |i, &j| i + j)
    });
}

#[bench]
fn bench_add_u8(b: &mut Bencher) {
    bench_add::<u8>(b);
}

#[bench]
fn bench_add_u16(b: &mut Bencher) {
    bench_add::<u16>(b);
}

#[bench]
fn bench_add_u32(b: &mut Bencher) {
    bench_add::<u32>(b);
}

#[bench]
fn bench_add_u64(b: &mut Bencher) {
    bench_add::<u64>(b);
}

#[bench]
fn bench_add_u128(b: &mut Bencher) {
    bench_add::<u128>(b);
}

fn bench_mul_xor<T>(b: &mut Bencher)
    where T: Copy + Default + Mul<Output=T> + BitXor<Output=T>, Standard: Distribution<T>
{
    let mut nums = [T::default(); 100];
    for i in nums.iter_mut() {
        *i = random();
    }
    b.iter(|| {
        nums.iter().fold(T::default(), |i, &j| (i * j) ^ j)
    });
}

#[bench]
fn bench_mul_xor_u8(b: &mut Bencher) {
    bench_mul_xor::<u8>(b);
}

#[bench]
fn bench_mul_xor_u16(b: &mut Bencher) {
    bench_mul_xor::<u16>(b);
}

#[bench]
fn bench_mul_xor_u32(b: &mut Bencher) {
    bench_mul_xor::<u32>(b);
}

#[bench]
fn bench_mul_xor_u64(b: &mut Bencher) {
    bench_mul_xor::<u64>(b);
}

#[bench]
fn bench_mul_xor_u128(b: &mut Bencher) {
    bench_mul_xor::<u128>(b);
}

#[bench]
fn bench_rem(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut nums = [0i8; 128];
    for i in nums.iter_mut() {
        *i = rng.gen_range(0, 30);
    }
    b.iter(|| {
        nums.iter().fold(0, |i, &j| (i + j) % 30)
    })
}

#[bench]
fn bench_rem_branch(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut nums = [0u8; 128];
    for i in nums.iter_mut() {
        *i = rng.gen_range(0, 30);
    }
    b.iter(|| {
        nums.iter().fold(0, |i, &j| {
            let sum = i + j;
            if sum < 30 {
                sum
            } else {
                sum - 30
            }
        })
    })
}
