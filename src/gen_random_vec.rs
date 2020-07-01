
use super::splitmix64;
use super::xorshift::xorshift_avx;

pub fn gen_random_vec(size: usize,mut seed: u64) -> Vec<u64>{
    let mut result = Vec::with_capacity(size);

    // initialize the seed
    let mut _seed: [u64; 4] =  [0; 4];
    for i in 0..4 {
        _seed[i] = splitmix64(seed);
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    }


    // fill fast most of the vector
    for _ in (0..size).step_by(4) {
        for v in &xorshift_avx(& mut _seed) {
            result.push(*v);
        }
    }

    // fill the remaining values
    let n = size -  (size % 4);
    match size % 4 {
        1 => {
            result[n] += result[n-1];
        },
        2 => {
            result[n] += result[n-1];
            result[n+1] += result[n];
        },
        3 => {
            result[n] += result[n-1];
            result[n+1] += result[n];
            result[n+2] += result[n+1];
        },
        _ => {},
    };

    result
}