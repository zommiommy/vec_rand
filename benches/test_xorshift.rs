#![feature(test)]
extern crate test;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use test::Bencher;

use vec_rand::xorshift::*;

const NUMBER: u64 = 1_000;

#[bench]
fn test_thread_rng(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    b.bytes = NUMBER * (std::mem::size_of::<u64>() as u64);
    b.iter(|| {
        for _ in 0..(32 * NUMBER) {
            rng.gen_range(0, 10000);
        }
        rng.gen_range(0, 10000)
    });
}

#[bench]
fn test_stdrng(b: &mut Bencher) {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0xBAD5EEDdeadbeef);
    b.bytes = NUMBER * (std::mem::size_of::<u64>() as u64);
    b.iter(|| {
        for _ in 0..(32 * NUMBER) {
            rng.gen_range(0, 10000);
        }
        rng.gen_range(0, 10000)
    });
}

#[bench]
fn test_xorshift(b: &mut Bencher) {
    let mut seed: u64 = 0xBAD5EEDdeadbeef;
    b.bytes = NUMBER * (std::mem::size_of::<u64>() as u64);
    b.iter(|| {
        for _ in 0..(32 * NUMBER) {
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
    b.bytes = NUMBER * (std::mem::size_of::<u64>() as u64);
    b.iter(|| {
        for _ in 0..(8 * NUMBER) {
            seed = xorshift_avx(&mut seed);
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
    b.bytes = NUMBER * (std::mem::size_of::<u64>() as u64);
    b.iter(|| {
        for _ in 0..(8 * NUMBER) {
            seed = xorshift_avx_intrinsics(&mut seed);
        }
        seed
    });
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_xorshift_avx_ss4(b: &mut Bencher) {
    let mut seed: [u64; 16] = [
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
    ];
    b.bytes = NUMBER * (std::mem::size_of::<u64>() as u64);
    b.iter(|| {
        for _ in 0..(2 * NUMBER) {
            seed = xorshift_avx_ss4(&mut seed);
        }
        seed
    });
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[bench]
fn test_xorshift_avx_ss8(b: &mut Bencher) {
    let mut seed: [u64; 32] = [
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
    ];
    b.bytes = NUMBER * (std::mem::size_of::<u64>() as u64);
    b.iter(|| {
        for _ in 0..(1 * NUMBER) {
            seed = xorshift_avx_ss8(&mut seed);
        }
        seed
    });
}
