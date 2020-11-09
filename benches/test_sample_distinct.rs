#![feature(test, asm)]
extern crate test;
use test::{Bencher, black_box};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

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
