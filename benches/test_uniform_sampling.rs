#![feature(test, asm)]
extern crate test;
use test::Bencher;

const NUMBER: u64 = 100000;

mod utils;
use utils::*;

use vec_rand::{
    sample_uniform, sample_uniform_unbiased_nearly_divisionless, sample_uniform_unbiased_simple,
    xorshift::xorshift,
};

#[bench]
fn test_nearly_divisionless(b: &mut Bencher) {
    let mut seed = 0x4201337deadbeef;
    b.iter(|| {
        let n = xorshift(seed);
        seed = xorshift(n);
        sample_uniform_unbiased_nearly_divisionless(n, seed)
    });
}

#[bench]
fn test_simple(b: &mut Bencher) {
    let mut seed = 0x4201337deadbeef;
    b.iter(|| {
        let n = xorshift(seed);
        seed = xorshift(n);
        sample_uniform_unbiased_simple(n, seed)
    });
}

#[bench]
fn test_biased(b: &mut Bencher) {
    let mut seed = 0x4201337deadbeef;
    b.iter(|| {
        let n = xorshift(seed);
        seed = xorshift(n);
        sample_uniform(n, seed)
    });
}
