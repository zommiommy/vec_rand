#![feature(test, asm)]
extern crate test;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use test::{black_box, Bencher};

const MAX: u64 = 2_000_000;
const QUANTITY: u64 = 10_000;

mod utils;
use utils::*;

use vec_rand::*;

#[bench]
fn test_sample_k_distinct_uniform_plain(b: &mut Bencher) {
    let seed = black_box(0xdeadbeefc0febabe);
    b.iter(|| sample_k_distinct_uniform_plain(0, MAX, QUANTITY, seed));
}

#[bench]
fn test_sample_k_distinct_uniform_naive(b: &mut Bencher) {
    let seed = black_box(0xdeadbeefc0febabe);
    b.iter(|| sample_k_distinct_uniform_naive(0, MAX, QUANTITY, seed));
}

#[bench]
fn test_sample_k_not_distinct_uniform_naive(b: &mut Bencher) {
    let seed = black_box(0xdeadbeefc0febabe);
    b.iter(|| sample_k_not_distinct_uniform_naive(0, MAX, QUANTITY, seed));
}

#[bench]
fn test_sample_k_distinct_uniform_btreeset(b: &mut Bencher) {
    let seed = black_box(0xdeadbeefc0febabe);
    b.iter(|| sample_k_distinct_uniform_btreeset(0, MAX, QUANTITY, seed));
}

#[bench]
fn test_sorted_unique_sub_sampling(b: &mut Bencher) {
    let seed = black_box(0xdeadbeefc0febabe);
    b.iter(|| sorted_unique_sub_sampling(0, MAX, QUANTITY, seed));
}
