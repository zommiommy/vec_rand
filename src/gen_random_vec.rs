
use super::splitmix64;
use super::xorshift::{
    xorshift_avx_ss8,
    xorshift_avx,
    xorshift
};

pub fn gen_random_vec_4_1(size: usize,mut seed: u64) -> Vec<u64>{
    let mut result = Vec::with_capacity(size);

    // initialize the seed
    let mut _seed: [u64; 4] =  [0; 4];
    for i in 0..4 {
        _seed[i] = splitmix64(seed);
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    }

    // fill fast most of the vector
    for _ in (0..(size-size%4)).step_by(4) {
        for v in &xorshift_avx(& mut _seed) {
            result.push(*v);
        }
    }

    // fill the remaining values
    let n = size -  (size % 4);

    for i in 0..(size % 4) {
        _seed[0] = xorshift(_seed[0]);
        result[n + i] += _seed[0];
    }

    result
}


pub fn gen_random_vec_32_4_1(size: usize,mut seed: u64) -> Vec<u64>{
    let mut result = Vec::with_capacity(size);

    // initialize the seed
    let mut _seed: [u64; 32] =  [0; 32];
    for i in 0..32 {
        _seed[i] = splitmix64(seed);
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    }

    // fill fast most of the vector
    for _ in (0..(size-size%32)).step_by(32) {
        for v in &xorshift_avx_ss8(& mut _seed) {
            result.push(*v);
        }
    }

    let mut _seed2: [u64; 4] = [0; 4];
    _seed2.copy_from_slice(&_seed[0..4]);
    // fill fast most of the vector
    for _ in (0..size).step_by(4) {
        for v in &xorshift_avx(& mut _seed2) {
            result.push(*v);
        }
    }

    for _ in (0..(size-size%4)).step_by(4) {
        _seed[0] = xorshift(_seed[0]);
        result.push(_seed[0]);
    }

    result
}


pub fn gen_random_vec_32_1(size: usize,mut seed: u64) -> Vec<u64>{
    let mut result = Vec::with_capacity(size);

    // initialize the seed
    let mut _seed: [u64; 32] =  [0; 32];
    for i in 0..32 {
        _seed[i] = splitmix64(seed);
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    }

    // fill fast most of the vector
    for _ in (0..(size-size%32)).step_by(32) {
        for v in &xorshift_avx_ss8(& mut _seed) {
            result.push(*v);
        }
    }

    for _ in 0..(size % 32) {
        _seed[0] = xorshift(_seed[0]);
        result.push(_seed[0]);
    }

    result
}


pub fn gen_random_vec_1(size: usize,mut seed: u64) -> Vec<u64>{
    let mut result = Vec::with_capacity(size);

    // fill fast most of the vector
    for _ in 0..size {
        seed = xorshift(seed);
        result.push(seed);   
    }

    result
}

