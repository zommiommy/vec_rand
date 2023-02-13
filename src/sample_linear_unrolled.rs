
use super::random_f64;

/// Given a vector of scores (non-zero positive values), convert it to a
/// probability distribution and extract a random indices accodringly.`
///
/// It useses cumsum_f64
#[inline]
pub fn sample_linear_unrolled(weights: &[f64], seed: u64) -> usize {
    if weights.len() == 1 {
        return 0;
    }

    let total: f64 = weights.iter().sum();
    let rnd: f64 = random_f64(seed) * total;

    let mut partial_total = 0.0;
    for i in (0..weights.len()).step_by(4) {
        let a = weights[i];
        let b = weights[i + 1];
        let c = weights[i + 2];
        let d = weights[i + 3];

        if rnd < partial_total + a {
            return i;
        }

        if rnd < partial_total + a + b {
            return i + 1;
        }

        if rnd < partial_total + a + b + c {
            return i + 2;
        }

        partial_total += a + b + c + d;

        if rnd < partial_total {
            return i + 3;
        }
    }
    panic!("Unreachable");
}
