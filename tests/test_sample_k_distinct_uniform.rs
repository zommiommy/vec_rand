#![feature(is_sorted)]

extern crate vec_rand;
use itertools::Itertools;

const START: usize = 5;
const END: usize = 21;
const ITER: usize = 1000;

#[test]
fn test_sample_k_distinct_uniform() {
    for seed in 1..ITER {
        for size in 1..(END - START + 1) {
            let sampled = vec_rand::sorted_unique_sub_sampling(
                START as u64,
                END as u64,
                size as u64,
                seed as u64 ^ 0xBAD5EED,
            )
            .unwrap();
            assert_eq!(sampled.len(), size);
            assert_eq!(sampled.len(), sampled.iter().unique().count());
            assert!((*sampled.first().unwrap() as usize) >= START);
            assert!((*sampled.last().unwrap() as usize) < END);
            assert!(sampled.is_sorted());
        }
    }
}
