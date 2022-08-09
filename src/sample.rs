use super::random_f64;
use super::cumsum_f64;
use core::result::Result::*;
use ::core::cmp::Ordering;

/// Given a vector of scores (non-zero positive values), convert it to a
/// probability distribution and extract a random indices accodringly.`
///
/// It useses cumsum_f64
pub fn sample(weights: &mut [f64], seed: u64) -> usize {
    if weights.len() == 1 {
        return 0;
    }

    cumsum_f64(weights);

    let rnd: f64 = random_f64(seed) * weights[weights.len() - 1];
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
        Err(g) => {
            // Due to float errors, the value of rnd might be slightly bigger
            // than the max. This would means that the returned value will be
            // weights.len(). But this is out of bounds so we want to cap it to
            // weights.len() - 1. The following code does this in a brenchless
            // way to reduce the brench missprediction penality
            let cond = g > weights.len() - 1;
            return (1 - cond as usize) * g + (cond as usize * (weights.len() - 1));
        }
    }
}
