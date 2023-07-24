mod cumsum_f32_plain;

use common_traits::Number;
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

pub fn cumsum<F: Number>(random_vec: &mut [F]) {
    #[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
    {
        //if is_x86_feature_detected!("sse") {
        //    cumsum_super_scaler_simd(random_vec);
        //    return;
        //}
    }
    cumsum_unrolled(random_vec)
}
