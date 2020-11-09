use super::*;


pub fn sample_k_distinct_uniform(min_value: u64, max_value: u64, quantity: u64, mut seed: u64) -> Vec<u64> {
    let mut extracted = Vec::with_capacity(quantity as usize);
    let step = (max_value - min_value) / quantity;
    seed = xorshift::xorshift(seed);
    let rnd = seed % step;
    let aligned = rnd !=0;
    if aligned {
        seed = xorshift::xorshift(seed);
        extracted.push(
            seed % rnd
        );
    }
    for i in aligned as u64..quantity - aligned as u64 {
        seed = xorshift::xorshift(seed);
        extracted.push(
            step * i + seed % step
        );
    }
    if aligned {
        seed = xorshift::xorshift(seed);
        let last_offset = step*(quantity-1);
        extracted.push(
            last_offset + seed % (max_value - last_offset)
        );
    }

    extracted
}   