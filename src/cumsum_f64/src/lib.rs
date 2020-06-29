#![feature(asm)]

mod cumsum_f64;
pub use cumsum_f64::cumsum_f64;

mod cumsum_f64_avx;
pub use cumsum_f64_avx::cumsum_f64_avx;

mod cumsum_f64_avx_intrinsics;
pub use cumsum_f64_avx_intrinsics::cumsum_f64_avx_intrinsics;

mod cumsum_f64_scan;
pub use cumsum_f64_scan::cumsum_f64_scan;

mod cumsum_f64_sse;
pub use cumsum_f64_sse::cumsum_f64_sse;

mod cumsum_f64_sse_intrinsics;
pub use cumsum_f64_sse_intrinsics::cumsum_f64_sse_intrinsics;

mod cumsum_f64_unrolled;
pub use cumsum_f64_unrolled::cumsum_f64_unrolled;