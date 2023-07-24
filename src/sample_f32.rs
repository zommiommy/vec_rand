use crate::cumsum;

use super::random_f32;
use ::core::cmp::Ordering;
use core::intrinsics::unlikely;
use core::panic;
use core::result::Result::*;

/// Given a vector of scores (non-zero positive values), convert it to a
/// probability distribution and extract a random indices accodringly.`
///
/// It useses cumsum_f64
#[inline]
pub fn sample_f32(weights: &mut [f32], seed: u64) -> usize {
    if unlikely(weights.len() == 0) {
        panic!("Called sample_f32 on a empty vector!!!");
    }
    if unlikely(weights.len() == 1) {
        return 0;
    }

    cumsum(weights);

    sample_from_cumsum(&weights, seed)
}

pub trait Primitive<T> {
    fn to(self) -> T;
}

impl Primitive<f32> for f32 {
    fn to(self) -> f32 {
        self
    }
}

impl Primitive<f64> for f32 {
    fn to(self) -> f64 {
        self as f64
    }
}

impl Primitive<u32> for f32 {
    fn to(self) -> u32 {
        self as u32
    }
}

impl Primitive<u64> for f32 {
    fn to(self) -> u64 {
        self as u64
    }
}

impl Primitive<i32> for f32 {
    fn to(self) -> i32 {
        self as i32
    }
}

impl Primitive<i64> for f32 {
    fn to(self) -> i64 {
        self as i64
    }
}

impl Primitive<usize> for f32 {
    fn to(self) -> usize {
        self as usize
    }
}

impl Primitive<isize> for f32 {
    fn to(self) -> isize {
        self as isize
    }
}

impl Primitive<f32> for u32 {
    fn to(self) -> f32 {
        self as f32
    }
}

/// Given a comulative sum of vector of scores (non-zero positive values), extracts a random indices accodringly.
#[inline]
pub fn sample_from_cumsum<F: PartialOrd<F> + Primitive<f32> + Copy>(
    comulative_sum: &[F],
    seed: u64,
) -> usize
where
    f32: Primitive<F>,
{
    if unlikely(comulative_sum.len() == 0) {
        panic!("Called sample_f32 on a empty vector!!!");
    }
    if unlikely(comulative_sum.len() == 1) {
        return 0;
    }

    let rnd: F =
        (random_f32(seed) * comulative_sum[comulative_sum.len().saturating_sub(1)].to()).to();

    // Find the first item which has a weight *higher* than the chosen weight.
    match comulative_sum.binary_search_by(|w| {
        if *w <= rnd {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }) {
        // this could be an unwrap_err but there is a small chance that
        // the value could exactly match one of the cumulative sums
        // and therefore return Ok.
        Ok(g) => g,
        Err(g) => core::cmp::min(g, comulative_sum.len() - 1),
    }
}
