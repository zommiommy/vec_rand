#![feature(test)]
extern crate test;
use test::Bencher;
use rand::Rng;

use vec_rand::xorshift::*;


#[bench]
fn test_thread_rng(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    b.iter(|| {
        rng.gen_range(0, 10000)
    });
}


#[bench]
fn test_xorshift(b: &mut Bencher) {
    let mut seed: u64 = 0xBAD5EEDdeadbeef;
    b.iter(|| {
        xorshift(& mut seed)
    });
}


#[bench]
fn test_xorshift_avx(b: &mut Bencher) {
    let mut seed: [u64; 4] = [
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
    ];
    b.iter(|| {
        xorshift_avx(& mut seed)
    });
}


#[bench]
fn test_xorshift_avx_intrinsics(b: &mut Bencher) {
    let mut seed: [u64; 4] = [
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef,
    ];
    b.iter(|| {
        xorshift_avx_intrinsics(& mut seed)
    });
}



#[bench]
fn test_xorshift_avx_ss4(b: &mut Bencher) {
    let mut seed: [u64; 16] = [
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
    ];
    b.iter(|| {
        xorshift_avx_ss4(& mut seed)
    });
}


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
        xorshift_avx_ss8(& mut seed)
    });
}
