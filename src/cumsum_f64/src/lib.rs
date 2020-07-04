#![feature(asm)]

mod cumsum_f64;
pub use cumsum_f64::cumsum_f64;

mod cumsum_f64_scan;
pub use cumsum_f64_scan::cumsum_f64_scan;

mod cumsum_f64_unrolled;
pub use cumsum_f64_unrolled::cumsum_f64_unrolled;

#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
mod cumsum_f64_sse;
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
pub use cumsum_f64_sse::cumsum_f64_sse;

#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
mod cumsum_f64_sse_intrinsics;
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
pub use cumsum_f64_sse_intrinsics::cumsum_f64_sse_intrinsics;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod cumsum_f64_avx;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use cumsum_f64_avx::cumsum_f64_avx;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod cumsum_f64_avx_intrinsics;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use cumsum_f64_avx_intrinsics::cumsum_f64_avx_intrinsics;
