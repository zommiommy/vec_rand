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
pub fn sample_f32_adapt(weights: &mut [f32], seed: u64) -> usize {
    if unlikely(weights.is_empty()) {
        panic!("Called sample_f32 on a empty vector!!!");
    }
    if unlikely(weights.len() == 1) {
        return 0;
    }

    cumsum(weights);

    let rnd: f32 = random_f32(seed) * weights[weights.len().saturating_sub(1)];

    if weights.len() > 100 {
        // Find the first item which has a weight *higher* than the chosen weight.
        match weights.binary_search_by(|w| {
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
            Err(g) => core::cmp::min(g, weights.len() - 1),
        }
    } else {
        let mut counter = 0;
        for w in weights.iter() {
            counter += (*w > rnd) as usize;
        }
        counter
    }
}
