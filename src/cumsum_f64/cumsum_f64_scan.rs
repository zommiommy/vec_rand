use core::iter::Iterator;
use core::option::Option::*;

#[cfg(feature="alloc")]
pub fn cumsum_f64_scan(random_vec: &[f64]) -> alloc::vec::Vec<f64> {
    random_vec
    .iter()
    .scan(0f64, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    })
    .collect()
}
