#![feature(asm)]

mod xorshift_avx;
pub use xorshift_avx::xorshift_avx;

mod xorshift_avx_ss4;
pub use xorshift_avx_ss4::xorshift_avx_ss4;

mod xorshift_avx_ss8;
pub use xorshift_avx_ss8::xorshift_avx_ss8;

mod xorshift_plain;
pub use xorshift_plain::xorshift_plain as xorshift;

mod xorshift_avx_intrinsics;
pub use xorshift_avx_intrinsics::xorshift_avx_intrinsics;