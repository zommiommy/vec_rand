#![feature(test, asm)]
extern crate test;
use test::Bencher;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand::{thread_rng};

const NUMBER: u64 = 100000;

mod utils;
use utils::*;

use vec_rand::*;

#[bench]
fn test_weighted_index_sample(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        WeightedIndex::new(&random_vec)
            .unwrap()
            .sample(&mut thread_rng())
    });
}


#[bench]
fn test_sample(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        sample(& mut random_vec)
    });
}

#[bench]
fn test_sample_avx(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        sample_avx(&random_vec)
    });
}
