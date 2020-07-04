#![feature(asm)]

mod xorshiro256plus_plain;
pub use xorshiro256plus_plain::xorshiro256plus_plain as xorshiro256plus;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod xorshiro256plus_avx;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use xorshiro256plus_avx::xorshiro256plus_avx;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod xorshiro256plus_avx_ss4;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub use xorshiro256plus_avx_ss4::xorshiro256plus_avx_ss4;