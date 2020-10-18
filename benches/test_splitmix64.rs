#![feature(test)]
extern crate test;
use test::Bencher;
use rand::Rng;

use vec_rand::splitmix64;

const NUM: u64 = 1_000;

#[bench]
fn test_thread_rng(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    b.iter(|| {
        for _ in 0..(32 * NUM) {
            rng.gen_range(0..10000);
        }
        rng.gen_range(0..10000)
    });
}


#[bench]
fn test_splitmix64(b: &mut Bencher) {
    let mut seed: u64 = 0xBAD5EEDdeadbeef;
    b.iter(|| {
        for _ in 0..(32 * NUM) {
            seed = splitmix64(seed);
        }
        seed
    });
}
