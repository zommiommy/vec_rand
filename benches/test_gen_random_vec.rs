#![feature(test, asm)]
extern crate test;
use test::Bencher;
use rand::Rng;

const NUMBER: u64 = 10000;

use vec_rand::xorshiro256plus::xorshiro256plus;
use vec_rand::xorshift::xorshift;

#[bench]
fn test_gen_range_of_thread_rng(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    b.iter(|| {
        (0..NUMBER).map(|_| rng.gen_range(0, NUMBER)).collect::<Vec<u64>>()
    });
}


#[bench]
fn test_with_xorshiro256plus(b: &mut Bencher) {
    let mut seed: [u64; 4] = [
        6591408588322595484,
        5451729388608518856,
        8913376598984957243,
        17912695770704705270
    ];

    b.iter(|| {
        (0..NUMBER).map(|_| xorshiro256plus(& mut seed) % NUMBER).collect::<Vec<u64>>()
    });
}

#[bench]
fn test_with_xorshift(b: &mut Bencher) {
    let mut seed: u64 = 6591408588322595484;

    b.iter(|| {
        (0..NUMBER).map(|_| {
            seed = xorshift(seed);
            seed % NUMBER
        }).collect::<Vec<u64>>()
    });
}