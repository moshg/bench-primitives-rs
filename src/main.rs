#![feature(test)]

extern crate rand;
extern crate test;


#[cfg(test)]
mod tests {
    use test::Bencher;

    use rand::prelude::*;

    #[bench]
    fn bench_u8_add(b: &mut Bencher) {
        let mut a = [0u8; 1000];
        for i in a.iter_mut() {
            *i = random();
        }
        b.iter(|| {
            a.iter().fold(0, |i, &j| i + j)
        })
    }

    #[bench]
    fn bench_u16_add(b: &mut Bencher) {
        let mut a = [0u16; 1000];
        for i in a.iter_mut() {
            *i = random::<u8>().into();
        }
        b.iter(|| {
            a.iter().fold(0, |i, &j| i + j)
        })
    }

    #[bench]
    fn bench_u32_add(b: &mut Bencher) {
        let mut a = [0u32; 1000];
        for i in a.iter_mut() {
            *i = random::<u8>().into();
        }
        b.iter(|| {
            a.iter().fold(0, |i, &j| i + j)
        })
    }

    #[bench]
    fn bench_u64_add(b: &mut Bencher) {
        let mut a = [0u64; 1000];
        for i in a.iter_mut() {
            *i = random::<u8>().into();
        }
        b.iter(|| {
            a.iter().fold(0, |i, &j| i + j)
        })
    }

    #[bench]
    fn bench_u128_add(b: &mut Bencher) {
        let mut a = [0u128; 1000];
        for i in a.iter_mut() {
            *i = random::<u8>().into();
        }
        b.iter(|| {
            a.iter().fold(0, |i, &j| i + j)
        })
    }

    #[bench]
    fn bench_u8(b: &mut Bencher) {
        let mut s = [0u8; 100];
        for i in s.iter_mut() {
            *i = random();
        }
        b.iter(|| {
            s.iter().fold(0, |i, &j| (i * j) ^ j)
        })
    }

    #[bench]
    fn bench_u16(b: &mut Bencher) {
        let mut s = [0u16; 100];
        for i in s.iter_mut() {
            *i = random::<u8>().into();
        }
        b.iter(|| {
            s.iter().fold(0, |i, &j| (i * j) ^ j)
        })
    }

    #[bench]
    fn bench_u32(b: &mut Bencher) {
        let mut s = [0u32; 100];
        for i in s.iter_mut() {
            *i = random::<u8>().into();
        }
        b.iter(|| {
            s.iter().fold(0, |i, &j| (i * j) ^ j)
        })
    }

    #[bench]
    fn bench_u64(b: &mut Bencher) {
        let mut s = [0u64; 100];
        for i in s.iter_mut() {
            *i = random::<u8>().into();
        }
        b.iter(|| {
            s.iter().fold(0, |i, &j| (i * j) ^ j)
        })
    }

    #[bench]
    fn bench_u128(b: &mut Bencher) {
        let mut s = [0u128; 100];
        for i in s.iter_mut() {
            *i = random::<u8>().into();
        }
        b.iter(|| {
            s.iter().fold(0, |i, &j| (i * j) ^ j)
        })
    }

    #[bench]
    fn bench_rem_if(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut a = [0u8; 128];
        for i in a.iter_mut() {
            *i = rng.gen_range(0, 30);
        }
        b.iter(|| {
            a.iter().fold(0, |i, &j| {
                let i = i + j;
                if i < 30 {
                    i
                } else {
                    i - 30
                }
            })
        })
    }

    #[bench]
    fn bench_rem(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut a = [0i8; 128];
        for i in a.iter_mut() {
            *i = rng.gen_range(-30, 30);
        }
        b.iter(|| {
            a.iter().fold(0, |i, &j| (i + j) % 30)
        })
    }
}
