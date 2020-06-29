#![feature(asm)]

mod cumsum_f32;
pub use cumsum_f32::cumsum_f32;

mod cumsum_f32_avx;
pub use cumsum_f32_avx::cumsum_f32_avx;

mod cumsum_f32_avx_intrinsics;
pub use cumsum_f32_avx_intrinsics::cumsum_f32_avx_intrinsics;

mod cumsum_f32_scan;
pub use cumsum_f32_scan::cumsum_f32_scan;

mod cumsum_f32_sse;
pub use cumsum_f32_sse::cumsum_f32_sse;

mod cumsum_f32_sse_intrinsics;
pub use cumsum_f32_sse_intrinsics::cumsum_f32_sse_intrinsics;

mod cumsum_f32_unrolled;
pub use cumsum_f32_unrolled::cumsum_f32_unrolled;