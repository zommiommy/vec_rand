#![feature(test, asm)]
extern crate test;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::thread_rng;
use test::Bencher;

const NUMBER: u64 = 10_000;
const ITERS: u64 = 1_000;

mod utils;
use utils::*;

use vec_rand::*;

#[bench]
fn test_weighted_index_sample(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        for _ in 0..ITERS {
            let _ = WeightedIndex::new(&random_vec)
                .unwrap()
                .sample(&mut thread_rng());
        }
    });
}

#[bench]
fn test_sample_f32(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER)
        .iter()
        .map(|x| *x as f32)
        .collect::<Vec<f32>>();
    b.iter(|| {
        for _ in 0..ITERS {
            let _ = sample_f32(&mut random_vec, 0xBad53eed);
        }
    });
}

#[bench]
fn test_sample_f32_adapt(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER)
        .iter()
        .map(|x| *x as f32)
        .collect::<Vec<f32>>();
    b.iter(|| {
        for _ in 0..ITERS {
            let _ = sample_f32_adapt(&mut random_vec, 0xBad53eed);
        }
    });
}

#[bench]
fn test_sample(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        for _ in 0..ITERS {
            let _ = sample(&mut random_vec, 0xBad53eed);
        }
    });
}

#[bench]
fn test_sample_plain(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        for _ in 0..ITERS {
            let _ = sample_plain(&mut random_vec, 0xBad53eed);
        }
    });
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_sample_avx(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        for _ in 0..ITERS {
            let _ = sample_avx(&mut random_vec, 0xBad53eed);
        }
    });
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_sample_modifing(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        for _ in 0..ITERS {
            let _ = sample_modifing(&mut random_vec, 0xBad53eed);
        }
    });
}
