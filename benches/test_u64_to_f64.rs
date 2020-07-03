#![feature(test)]
extern crate test;
use test::Bencher;

use vec_rand;

mod utils;
use utils::*;

const NUMBER: u64 = 10000;

#[bench]
fn test_u64_to_f64_mul(b: &mut Bencher) {
    let random_vec = gen_random_u64_vec(NUMBER);
    b.iter(|| {
        let mut result = Vec::with_capacity(random_vec.len());
        for x in &random_vec {
            result.push(vec_rand::u64_to_f64_mul(*x));
        }
    });
}

#[bench]
fn test_u64_to_f64_no_mul(b: &mut Bencher) {
    let random_vec = gen_random_u64_vec(NUMBER);
    b.iter(|| {
        let mut result = Vec::with_capacity(random_vec.len());
        for x in &random_vec {
            result.push(vec_rand::u64_to_f64_no_mul(*x));
        }
    });
}
