
use super::{
    u64_to_f64,
    gen_random_vec
};

pub fn gen_random_vec_f64(size: usize, seed: u64) -> Vec<f64>{
    gen_random_vec(size, seed).iter().map(
        |x|
            u64_to_f64(*x)
    ).collect()
}

pub fn gen_random_vec_f32(size: usize, seed: u64) -> Vec<f32>{
    gen_random_vec(size, seed).iter().map(
        |x|
            u64_to_f64(*x) as f32
    ).collect()
}