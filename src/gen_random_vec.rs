use super::xorshift::xorshift;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use super::xorshift::{xorshift_avx, xorshift_avx_ss8};

#[cfg(feature = "alloc")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub fn gen_random_vec_4_1(size: usize, mut seed: u64) -> Vec<u64> {
    let mut result = Vec::with_capacity(size);

    use super::splitmix64;
    // initialize the seed
    let mut vec_seed: [u64; 4] = [0; 4];
    for s in vec_seed.iter_mut() {
        *s = splitmix64(seed);
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    }

    // fill fast most of the vector
    for _ in (0..(size - size % 4)).step_by(4) {
        for v in &xorshift_avx(&mut vec_seed) {
            result.push(*v);
        }
    }
    // fill the remaining values

    for _ in 0..(size % 4) {
        vec_seed[0] = xorshift(vec_seed[0]);
        result.push(vec_seed[0]);
    }

    result
}

#[cfg(feature = "alloc")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub fn gen_random_vec_32_4_1(size: usize, mut seed: u64) -> Vec<u64> {
    let mut result = Vec::with_capacity(size);

    use super::splitmix64;
    // initialize the seed
    let mut vec_seed: [u64; 32] = [0; 32];
    for s in vec_seed.iter_mut() {
        *s = splitmix64(seed);
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    }

    // fill fast most of the vector
    for _ in (0..(size - size % 32)).step_by(32) {
        for v in &xorshift_avx_ss8(&mut vec_seed) {
            result.push(*v);
        }
    }

    let mut vec_seed2: [u64; 4] = [0; 4];
    vec_seed2.copy_from_slice(&vec_seed[0..4]);
    // fill fast most of the vector
    for _ in (0..size % 32 - size % 4).step_by(4) {
        for v in &xorshift_avx(&mut vec_seed2) {
            result.push(*v);
        }
    }

    for _ in 0..size % 4 {
        vec_seed[0] = xorshift(vec_seed[0]);
        result.push(vec_seed[0]);
    }

    result
}

#[cfg(feature = "alloc")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub fn gen_random_vec_32_1(size: usize, mut seed: u64) -> Vec<u64> {
    let mut result = Vec::with_capacity(size);

    use super::splitmix64;
    // initialize the seed
    let mut vecvec_seed: [u64; 32] = [0; 32];
    for s in vecvec_seed.iter_mut() {
        *s = splitmix64(seed);
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    }

    // fill fast most of the vector
    for _ in (0..(size - size % 32)).step_by(32) {
        for v in &xorshift_avx_ss8(&mut vecvec_seed) {
            result.push(*v);
        }
    }

    for _ in 0..(size % 32) {
        vecvec_seed[0] = xorshift(vecvec_seed[0]);
        result.push(vecvec_seed[0]);
    }

    result
}

#[cfg(feature = "alloc")]
#[inline]
pub fn gen_random_vec_1(size: usize, mut seed: u64) -> Vec<u64> {
    let mut result = Vec::with_capacity(size);

    // fill fast most of the vector
    for _ in 0..size {
        seed = xorshift(seed);
        result.push(seed);
    }

    result
}
