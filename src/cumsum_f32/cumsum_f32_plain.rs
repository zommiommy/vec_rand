#[cfg(feature = "alloc")]
#[inline]
pub fn cumsum_f32_plain(random_vec: &[f32]) -> alloc::vec::Vec<f32> {
    let mut cumulative_sum = alloc::vec::Vec::with_capacity(random_vec.len());
    let mut total_weight = 0f32;
    for w in random_vec {
        total_weight += w;
        cumulative_sum.push(total_weight);
    }
    cumulative_sum
}
