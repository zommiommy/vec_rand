
use ::core::cmp::Ordering;
use super::random_f64;

use cumsum_f64::cumsum_f64;

/// Given a vector of scores (non-zero positive values), convert it to a
/// probability distribution and extract a random indices accodringly.`
///
/// It useses cumsum_f64
pub fn sample(weights: & Vec<f64>) -> usize {
    if weights.len() == 1 {
        return 0;
    }

    let cumsum = cumsum_f64(&weights);

    let rnd: f64 = random_f64() * cumsum[cumsum.len() - 1];

    // Find the first item which has a weight *higher* than the chosen weight.
    match cumsum.binary_search_by(|w| {
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
