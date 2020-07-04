#![feature(asm)]

mod cumsum_f32;
pub use cumsum_f32::cumsum_f32;

mod cumsum_f32_scan;
pub use cumsum_f32_scan::cumsum_f32_scan;

mod cumsum_f32_unrolled;
pub use cumsum_f32_unrolled::cumsum_f32_unrolled;




#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
mod cumsum_f32_sse;
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
pub use cumsum_f32_sse::cumsum_f32_sse;

#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
mod cumsum_f32_sse_intrinsics;
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
pub use cumsum_f32_sse_intrinsics::cumsum_f32_sse_intrinsics;


#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod cumsum_f32_avx;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use cumsum_f32_avx::cumsum_f32_avx;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod cumsum_f32_avx_intrinsics;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use cumsum_f32_avx_intrinsics::cumsum_f32_avx_intrinsics;