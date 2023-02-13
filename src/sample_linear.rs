
use super::random_f64;

/// Given a vector of scores (non-zero positive values), convert it to a
/// probability distribution and extract a random indices accodringly.`
///
/// It useses cumsum_f64
#[inline]
pub fn sample_linear(weights: &[f64], seed: u64) -> usize {
    if weights.len() == 1 {
        return 0;
    }

    let total: f64 = weights.iter().sum();
    let rnd: f64 = random_f64(seed) * total;

    let mut partial_total = 0.0;
    for (i, w) in weights.iter().enumerate() {
        partial_total += w;
        if rnd < partial_total {
            return i;
        }
    }
    panic!("Unreachable");
}
