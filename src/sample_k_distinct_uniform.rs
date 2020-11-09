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


pub fn sample_k_distinct_uniform_naive(
    min_value: u64,
    max_value: u64,
    quantity: u64,
    mut seed: u64,
) -> Vec<u64> {
    let mut extracted = Vec::with_capacity(quantity as usize);
    let delta = max_value - min_value;
    let step = delta / quantity;
    seed = xorshift::xorshift(seed);
    let rnd = seed % step;
    let aligned = rnd != 0;
    if aligned {
        seed = xorshift::xorshift(seed);
        extracted.push(min_value + seed % rnd);
    }
    for i in aligned as u64..quantity - aligned as u64 {
        seed = xorshift::xorshift(seed);
        extracted.push(min_value + rnd + step * i + seed % step);
    }
    if aligned {
        seed = xorshift::xorshift(seed);
        let last_offset = rnd + step * (quantity - 1);
        extracted.push(min_value + last_offset + seed % (delta - last_offset));
    }

    extracted
}
