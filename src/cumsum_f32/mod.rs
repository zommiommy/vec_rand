mod cumsum_f32_plain;
use core::ops::{Add, AddAssign};

#[cfg(feature = "alloc")]
pub use cumsum_f32_plain::cumsum_f32_plain;

mod cumsum_f32_scan;
#[cfg(feature = "alloc")]
pub use cumsum_f32_scan::cumsum_f32_scan;

mod cumsum_unrolled;
pub use cumsum_unrolled::cumsum_unrolled;

#[cfg(target_arch = "x86_64")]
mod cumsum_f32_sse_intrinsics;
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "alloc")]
pub use cumsum_f32_sse_intrinsics::cumsum_f32_sse_intrinsics;

#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
mod cumsum_super_scalar_simd;
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
pub use cumsum_super_scalar_simd::cumsum_super_scaler_simd;

use self::cumsum_unrolled::Zero;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
extern "C" {
    fn prefix_sum_AVX256(array: *mut f32, len: usize);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn the_stackoverflow_thingy_256(array: &mut [f32], len: usize) {
    unsafe { prefix_sum_AVX256(array.as_mut_ptr(), len) }
}

pub fn cumsum<F: Zero + AddAssign + Add<F, Output = F> + Copy>(random_vec: &mut [F]) {
    #[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
    {
        //if is_x86_feature_detected!("sse") {
        //    cumsum_super_scaler_simd(random_vec);
        //    return;
        //}
    }
    cumsum_unrolled(random_vec)
}
