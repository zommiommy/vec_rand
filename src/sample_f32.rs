use super::random_f32;
use ::core::cmp::Ordering;

use super::cumsum_f32::cumsum_f32;

/// Given a vector of scores (non-zero positive values), convert it to a
/// probability distribution and extract a random indices accodringly.`
///
/// It useses cumsum_f64
pub fn sample_f32(weights: &mut Vec<f32>, seed: u64) -> usize {
    if weights.len() == 0 {
        panic!("Called sample_f32 on a empty vector!!!");
    }
    if weights.len() == 1 {
        return 0;
    }

    cumsum_f32(weights);

    let rnd: f32 = random_f32(seed) * weights[weights.len().saturating_sub(1)];

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
        Err(g) => std::cmp::min(g, weights.len() - 1),
    }
}
