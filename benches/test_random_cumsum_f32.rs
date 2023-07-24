#![feature(test)]
extern crate test;
use rand::Rng;
use test::Bencher;

use vec_rand::cumsum_f32::*;

mod utils;
use utils::*;

const ITER_SMALL: u64 = 1_000_000;
const ITER_BIG: u64 = 10_000;
const ITER_SEARCH: u64 = 10_000;
const SMALL: u64 = 9;
const BIG: u64 = 1000;
const SEARCH: u64 = 1000;

macro_rules! unroll_value {
    ($data:expr, $($val:literal),*) => {{
        match $data.len() {
            0 | 1=> {}
            $(
                $val => {
                    for i in 1..$val {
                        $data[i] += $data[i - 1];
                    }
                }
            )*
            _ => {
                cumsum_f32(&mut $data);
            }
        }
    }};
}

#[bench]
fn test_dispatching_small(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ITER_SMALL {
            let mut values = gen_random_f32_vec_random_len(SMALL);
            unroll_value!(
                values, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23
            );
        }
    });
}

#[bench]
fn test_naive_small(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ITER_SMALL {
            let mut values = gen_random_f32_vec_random_len(SMALL);
            cumsum_f32(&mut values);
        }
    });
}

#[bench]
fn test_dispatching_big(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ITER_BIG {
            let mut values = gen_random_f32_vec_random_len(BIG);
            unroll_value!(
                values, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23
            );
        }
    });
}

#[bench]
fn test_naive_big(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ITER_BIG {
            let mut values = gen_random_f32_vec_random_len(BIG);
            cumsum_f32(&mut values);
        }
    })
}

fn overhead() -> (Vec<u64>, u64) {
    let mut values = gen_random_u64_vec_random_len(SEARCH);
    values.sort();

    let mut rng = rand::thread_rng();
    let val_to_find = rng.gen_range(0, SEARCH);

    (values, val_to_find)
}

fn linear() -> usize {
    let (values, val_to_find) = overhead();
    for (i, value) in values.iter().enumerate() {
        if *value == val_to_find {
            return i;
        }
    }
    0
}

fn binary_search() -> Result<usize, usize> {
    let (values, val_to_find) = overhead();
    values.binary_search(&val_to_find)
}

#[bench]
fn bench_overhead(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ITER_SEARCH {
            let _ = overhead();
        }
    })
}

#[bench]
fn bench_linear_scan(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ITER_SEARCH {
            let _ = linear();
        }
    })
}

#[bench]
fn bench_binary_search(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ITER_SEARCH {
            let _ = binary_search();
        }
    })
}
