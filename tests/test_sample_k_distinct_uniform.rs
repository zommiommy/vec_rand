#![feature(is_sorted)]

extern crate vec_rand;
use itertools::Itertools;

const START: usize = 5;
const END: usize = 21;
const ITER: usize = 1000;

#[test]
fn test_sample_k_distinct_uniform() {
    for seed in 1..ITER {
        for size in START..END {
            let sampled = vec_rand::sample_k_distinct_uniform(
                START as u64,
                (END * 5) as u64,
                (size + 1) as u64,
                seed as u64 ^ 0xBAD5EED,
            );
            assert_eq!(sampled.len(), sampled.iter().unique().count());
            assert!(sampled.is_sorted());
        }
    }
}
