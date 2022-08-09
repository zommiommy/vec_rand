use super::{gen_random_vec, u64_to_f64};
use core::iter::Iterator;

#[cfg(feature="alloc")]
use alloc::vec::Vec;

#[cfg(feature="alloc")]
pub fn gen_random_vec_f64(size: usize, seed: u64) -> Vec<f64> {
    gen_random_vec(size, seed)
        .iter()
        .map(|x| u64_to_f64(*x))
        .collect()
}

#[cfg(feature="alloc")]
pub fn gen_random_vec_f32(size: usize, seed: u64) -> Vec<f32> {
    gen_random_vec(size, seed)
        .iter()
        .map(|x| u64_to_f64(*x) as f32)
        .collect()
}
