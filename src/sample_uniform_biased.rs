
use super::xorshift::xorshift;

/// Return random number from a uniform distribution [0, number).
/// 
/// # Arguments
/// * number, the upperlimit of the values
/// * seed, the seed to use to generate the number 
pub fn sample_uniform_biased(number: u64, seed: u64) -> usize {
    (xorshift(seed) % number) as usize
}