
use ::core::cmp::Ordering;
use super::random_f64;

/// Given a vector of scores (non-zero positive values), convert it to a
/// probability distribution and extract a random indices accodringly.`
///
/// # Implementation details
/// The implemented method is O(n) because the first operations is to calculate
/// the cumulative sum of the weights, then we extract a random floating value
/// between 0 and the last value of the cumulative sum.
/// Finally, we find the index of the first value bigger than it by binary search.
///
///
/// The AVX / SSE implementation for the cumulative sum are faster for large arrays
/// But on small vectors the naife implementations is faster.
#[inline]
pub fn sample_unsafe(weights: &mut Vec<f64>, seed: u64) -> usize {
    if weights.len() == 1 {
        return 0;
    }

    // this method is generally slower than the sse version implemented in the benchmarks,
    // but in our graph we often have slow
    let mut total_weight = 0f64;

    for i in 0..weights.len() {
        unsafe{
            let w = weights.get_unchecked_mut(i);
            total_weight += *w;
            *w = total_weight;
        }
    }

    let rnd: f64 = random_f64(seed) * total_weight;

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
        Err(g) => g,
    }
}
