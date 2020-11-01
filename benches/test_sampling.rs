#![feature(test, asm)]
extern crate test;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::thread_rng;
use test::Bencher;

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
fn test_sample_f32(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER).iter().map(|x| *x as f32).collect::<Vec<f32>>();
    b.iter(|| sample_f32(&mut random_vec, 0xBad53eed));
}

#[bench]
fn test_sample(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| sample(&mut random_vec, 0xBad53eed));
}

#[bench]
fn test_sample_plain(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| sample_plain(&mut random_vec, 0xBad53eed));
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_sample_avx(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| sample_avx(&mut random_vec, 0xBad53eed));
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_sample_modifing(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| sample_modifing(&mut random_vec, 0xBad53eed));
}
