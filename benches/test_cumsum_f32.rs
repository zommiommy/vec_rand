#![feature(test)]
extern crate test;
use test::Bencher;

use vec_rand::cumsum_f32::*;

mod utils;
use utils::*;

const NUMBER: u64 = 32 * 10000;

#[bench]
fn test_cumsum_f32(b: &mut Bencher) {
    let mut random_vec = gen_random_f32_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f32>() as u64);
    b.iter(|| cumsum_f32(&mut random_vec));
}

#[bench]
fn test_cumsum_f32_plain(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f32>() as u64);
    b.iter(|| cumsum_f32_plain(&random_vec));
}

#[bench]
fn test_cumsum_f32_scan(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f32>() as u64);
    b.iter(|| cumsum_f32_scan(&random_vec));
}

#[bench]
fn test_cumsum_f32_sse_intrinsics(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f32>() as u64);
    b.iter(|| cumsum_f32_sse_intrinsics(&random_vec));
}

#[bench]
fn test_cumsum_super_scaler_simd(b: &mut Bencher) {
    let mut random_vec = gen_random_f32_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f32>() as u64);
    b.iter(|| cumsum_super_scaler_simd(&mut random_vec));
}

#[bench]
fn test_cumsum_f32_unrolled(b: &mut Bencher) {
    let mut random_vec = gen_random_f32_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f32>() as u64);
    b.iter(|| cumsum_f32_unrolled(&mut random_vec));
}
