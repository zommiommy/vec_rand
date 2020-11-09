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
fn test_sample_k_distinct_uniform(b: &mut Bencher) {
    let seed = black_box(0xdeadbeefc0febabe);
    b.iter(|| sample_k_distinct_uniform(0, MAX, QUANTITY, seed));
}

#[bench]
fn test_sorting(b: &mut Bencher) {
    let seed = black_box(0xdeadbeefc0febabeu64);
    b.iter(|| {
        let mut rnd = SmallRng::seed_from_u64((seed) as u64);
        let mut indices = (0..MAX).collect::<Vec<u64>>();
        indices.shuffle(&mut rnd);
        indices[0..QUANTITY as usize].to_vec()
    });
}
