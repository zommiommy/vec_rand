#![feature(asm)]

mod xorshiro256plus_avx;
pub use xorshiro256plus_avx::xorshiro256plus_avx;

mod xorshiro256plus_avx_ss4;
pub use xorshiro256plus_avx_ss4::xorshiro256plus_avx_ss4;

mod xorshiro256plus_plain;
pub use xorshiro256plus_plain::xorshiro256plus_plain as xorshiro256plus;