use common_traits::Number;

#[inline]
/// Computes implace the cumulative sum of a vector of additive values.
///
/// # Example
///
/// ```
/// use vec_rand::cumsum_f32::cumsum_unrolled;
///
/// let mut weights = vec![1.0, 2.0, 3.0, 4.0];
/// cumsum_unrolled(&mut weights);
///
/// assert_eq!(weights, vec![1.0, 3.0, 6.0, 10.0]);
/// ```
pub fn cumsum_unrolled<F: Number>(random_vec: &mut [F]) {
    if random_vec.is_empty() {
        return;
    }

    let mut offset = F::ZERO;

    let max = if random_vec.len() >= 4 {
        if random_vec.len() % 4 == 0 {
            random_vec.len()
        } else {
            random_vec.len() - 4
        }
    } else {
        0
    };
    for i in (0..max).step_by(4) {
        let mut a = random_vec[i];
        let mut b = random_vec[i + 1];
        let mut c = random_vec[i + 2];
        let mut d = random_vec[i + 3];

        d += c + b + a + offset;
        c += b + a + offset;
        b += a + offset;
        a += offset;

        random_vec[i] = a;
        random_vec[i + 1] = b;
        random_vec[i + 2] = c;
        random_vec[i + 3] = d;

        offset = d;
    }

    let n = random_vec.len() - (random_vec.len() % 4);
    match random_vec.len() % 4 {
        1 => {
            random_vec[n] += offset;
        }
        2 => {
            random_vec[n] += offset;
            random_vec[n + 1] += random_vec[n];
        }
        3 => {
            random_vec[n] += offset;
            random_vec[n + 1] += random_vec[n];
            random_vec[n + 2] += random_vec[n + 1];
        }
        _ => {}
    };
}
