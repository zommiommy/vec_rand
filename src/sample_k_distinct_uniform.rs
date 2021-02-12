use super::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::collections::BTreeSet;

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


pub fn sample_k_not_distinct_uniform_naive(
    min_value: u64,
    max_value: u64,
    quantity: u64,
    mut seed: u64,
) -> Vec<u64> {
    let delta = max_value - min_value;
    let mut result = Vec::with_capacity(quantity as usize);
    for _ in 0..quantity {
        seed = xorshift::xorshift(seed);
        let rnd = min_value + seed % delta;
        result.push(rnd);
    }
    result.sort();
    result
}

pub fn sample_k_distinct_uniform_naive(
    min_value: u64,
    max_value: u64,
    quantity: u64,
    mut seed: u64,
) -> Vec<u64> {
    let delta = max_value - min_value;
    let mut result = Vec::with_capacity(quantity as usize);
    for _ in 0..quantity {
        'inner: loop {
            seed = xorshift::xorshift(seed);
            let rnd = min_value + seed % delta;
            if !result.iter().any(|&i| i==rnd) {
                result.push(rnd);
                break 'inner;
            }
        }
    }
    result.sort();
    result
}


pub fn sample_k_distinct_uniform_btreeset(
    min_value: u64,
    max_value: u64,
    quantity: u64,
    mut seed: u64,
) -> Vec<u64> {
    let delta = max_value - min_value;
    let mut result = BTreeSet::new();
    for _ in 0..quantity {
        'inner: loop {
            seed = xorshift::xorshift(seed);
            let rnd = min_value + seed % delta;
            if result.get(&rnd).is_none() {
                result.insert(rnd);
                break 'inner;
            }
        }
    }
    result.iter().cloned().collect::<Vec<u64>>()
}


/// Returns a sampled vector of `quantity` sorted unique integer values 
/// in [`min_value`, `max_value`).
/// 
/// # The Algorithm, its limitations and biases
/// This algorithm uses a simple approximation, the domain is divied in 
/// `quantity` buckets of equal size (except the last one) and a random value
/// is sampled from each bucket, this way we can extract sorted and distincted values.
/// Since `max_value - min_value` might not be a multiple of `quantity` we add any
/// spare domain to the last bucket, so the probability that each value extracted
/// is approximately uniform, with a negative bias torwards the "higher" values.
/// This approach has several limitations such that most paris of consecutive values
/// cannot be extracted. Thus while being approximatively uniform it has much less
/// entroy compared to a true vector of sorted values.
///
/// Thus this cannot be used for any cryptografically robust application (also because
/// it's based on xorshift which is not a crypto-safe PRNGt) but it's meant for 
/// applications such as monte-carlo.
///
/// In particular it's great for extracting sets of values, since sets can be 
/// represented as a sorted list of integers (like in roaring bitmaps).
///
/// # Arguments
///
/// * min_value: u64 - Minimum value of the sampling range.
/// * max_value: u64 - Maximum value of the sampling range.
/// * quantity: u64 - Number of values to be sampled.
/// * mut seed: u64 - Seed to reproduce the sampling.
///
pub fn sorted_unique_sub_sampling(
    min_value: u64,
    max_value: u64,
    quantity: u64,
    mut seed: u64,
) -> Result<Vec<u64>, String> {
    // compute the size of the range of values
    let delta = max_value.saturating_sub(min_value);
    if quantity > delta {
        return Err(format!(
            "Required quantity {} is greater than given range size {}.",
            quantity, delta
        ));
    }
    // allocate the result vector
    let mut extracted = Vec::with_capacity(quantity as usize);
    
    // Compute the size of the buckets
    let step = delta / quantity;
    
    // From each bucket extract a random value
    for i in 0..quantity.saturating_sub(1) {
        // in release build the step * i should be replaced
        // by an accumulator register and thus don't actually 
        extracted.push(min_value + step * i + seed % step);
        seed = xorshift::xorshift(seed);
    }
    
    // Extract the random value from the last bucket
    // here we add any spare range (step is floor(delta / quantity)
    // Therefore the last values will be slightly less probable
    // and thus there is a slight bias.
    //
    // TODO (NOT PRIORITY): figure out if the compiler actually does the multiplication or
    // is smart enough to figure it out from the accumulator register 
    // or it actually does the multiplication.
    extracted.push(max_value - seed % (delta - step * (quantity - 1)) - 1);
    
    Ok(extracted)
}
