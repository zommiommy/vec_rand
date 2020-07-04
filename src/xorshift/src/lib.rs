#![feature(asm)]

mod xorshift_plain;
pub use xorshift_plain::xorshift_plain as xorshift;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod xorshift_avx_intrinsics;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use xorshift_avx_intrinsics::xorshift_avx_intrinsics;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod xorshift_avx;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use xorshift_avx::xorshift_avx;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod xorshift_avx_ss4;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use xorshift_avx_ss4::xorshift_avx_ss4;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod xorshift_avx_ss8;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use xorshift_avx_ss8::xorshift_avx_ss8;
