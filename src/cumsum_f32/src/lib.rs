#![feature(asm)]

mod cumsum_f32_plain;
pub use cumsum_f32_plain::cumsum_f32_plain;

mod cumsum_f32_scan;
pub use cumsum_f32_scan::cumsum_f32_scan;

mod cumsum_f32_unrolled;
pub use cumsum_f32_unrolled::cumsum_f32_unrolled;




#[cfg(target_arch = "x86_64")]
mod cumsum_f32_sse;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f32_sse::cumsum_f32_sse;

#[cfg(target_arch = "x86_64")]
mod cumsum_f32_sse_intrinsics;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f32_sse_intrinsics::cumsum_f32_sse_intrinsics;


#[cfg(target_arch = "x86_64")]
mod cumsum_f32_avx;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f32_avx::cumsum_f32_avx;

#[cfg(target_arch = "x86_64")]
mod cumsum_f32_avx_intrinsics;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f32_avx_intrinsics::cumsum_f32_avx_intrinsics;


pub fn cumsum_f32(random_vec: &Vec<f32>) -> Vec<f32>{
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse") {
            return cumsum_f32_sse_intrinsics(random_vec);
        }
    }
    cumsum_f32_unrolled(random_vec)
}