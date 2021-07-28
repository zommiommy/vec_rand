#![feature(test)]
extern crate test;
use test::Bencher;

use vec_rand::cumsum_f64::*;

mod utils;
use utils::*;

const NUMBER: u64 = 32 * 10000;

#[bench]
fn test_cumsum_f64(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f64>() as u64);
    b.iter(|| cumsum_f64(&mut random_vec));
}

#[bench]
fn test_cumsum_f64_plain(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f64>() as u64);
    b.iter(|| cumsum_f64_plain(&random_vec));
}

#[bench]
fn test_cumsum_f64_scan(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f64>() as u64);
    b.iter(|| cumsum_f64_scan(&random_vec));
}

#[bench]
fn test_cumsum_f64_sse_intrinsics(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f64>() as u64);
    b.iter(|| cumsum_f64_sse_intrinsics(&mut random_vec));
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_cumsum_f64_avx_intrinsics(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f64>() as u64);
    b.iter(|| cumsum_f64_avx_intrinsics(&mut random_vec));
}

#[bench]
fn test_cumsum_f64_unrolled(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.bytes = NUMBER * (std::mem::size_of::<f64>() as u64);
    b.iter(|| cumsum_f64_unrolled(&mut random_vec));
}
