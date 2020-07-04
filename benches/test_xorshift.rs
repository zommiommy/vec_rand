#![feature(test)]
extern crate test;
use test::Bencher;
use rand::Rng;

use vec_rand::xorshift::*;

const NUM: u64 = 1_000;

#[bench]
fn test_thread_rng(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    b.iter(|| {
        for _ in 0..(32 * NUM) {
            rng.gen_range(0, 10000);
        }
        rng.gen_range(0, 10000)
    });
}


#[bench]
fn test_xorshift(b: &mut Bencher) {
    let mut seed: u64 = 0xBAD5EEDdeadbeef;
    b.iter(|| {
        for _ in 0..(32 * NUM) {
            seed = xorshift(seed);
        }
        seed
    });
}


#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_xorshift_avx(b: &mut Bencher) {
    let mut seed: [u64; 4] = [
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
    ];
    b.iter(|| {
        for _ in 0..(8 * NUM) {
            xorshift_avx(& mut seed);
        }
        seed
    });
}


#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_xorshift_avx_intrinsics(b: &mut Bencher) {
    let mut seed: [u64; 4] = [
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
    ];
    b.iter(|| {
        for _ in 0..(8 * NUM) {
            xorshift_avx_intrinsics(& mut seed);
        }
        seed
    });
}



#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_xorshift_avx_ss4(b: &mut Bencher) {
    let mut seed: [u64; 16] = [
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
    ];
    b.iter(|| {
        for _ in 0..(2 * NUM) {
            xorshift_avx_ss4(& mut seed);
        }
        seed
    });
}


#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_xorshift_avx_ss8(b: &mut Bencher) {
    let mut seed: [u64; 32] = [
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
    ];
    b.iter(|| {
        for _ in 0..(1 * NUM) {
            xorshift_avx_ss8(& mut seed);
        }
        seed
    });
}
