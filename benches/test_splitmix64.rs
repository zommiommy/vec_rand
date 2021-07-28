#![feature(test)]
extern crate test;
use rand::Rng;
use test::Bencher;

use vec_rand::splitmix64;

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
fn test_splitmix64(b: &mut Bencher) {
    let mut seed: u64 = 0xBAD5EEDdeadbeef;
    b.bytes = NUMBER * (std::mem::size_of::<u64>() as u64);
    b.iter(|| {
        for _ in 0..(32 * NUMBER) {
            seed = splitmix64(seed);
        }
        seed
    });
}
