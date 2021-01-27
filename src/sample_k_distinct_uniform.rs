use super::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub fn sample_k_distinct_uniform_plain(
    min_value: u64,
    max_value: u64,
    quantity: u64,
    seed: u64,
) -> Vec<u64> {
    let mut rnd = SmallRng::seed_from_u64((seed) as u64);
    let mut indices = (min_value..max_value).collect::<Vec<u64>>();
    indices.shuffle(&mut rnd);

    indices[0..quantity as usize].to_vec()
}

/// Set the column_number of the nodes.
///
/// # Arguments
///
/// * min_value: u64,
/// * max_value: u64,
/// * quantity: u64,
/// * mut seed: u64,
///
pub fn sorted_unique_sub_sampling(
    min_value: u64,
    max_value: u64,
    quantity: u64,
    mut seed: u64,
) -> Result<Vec<u64>, String> {
    let delta = max_value - min_value;
    if quantity > delta {
        return Err(format!(
            "Required quantity {} is greater than given range size {}.",
            quantity, delta
        ));
    }
    let mut extracted = Vec::with_capacity(quantity as usize);
    let step = delta / quantity;
    for i in 0..quantity - 1 {
        seed = xorshift::xorshift(seed);
        extracted.push(min_value + step * i + seed % step);
    }
    extracted.push(max_value - seed % (delta - step * (quantity - 1)) - 1);
    Ok(extracted)
}
