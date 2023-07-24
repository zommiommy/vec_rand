use core::iter::Iterator;
use core::option::Option::*;

#[cfg(feature = "alloc")]
#[inline]
pub fn cumsum_f32_scan(random_vec: &[f32]) -> alloc::vec::Vec<f32> {
    random_vec
        .iter()
        .scan(0f32, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}
