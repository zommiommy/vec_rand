mod xorshift_plain;
pub use xorshift_plain::xorshift_plain as xorshift;

#[cfg(target_arch = "x86_64")]
mod xorshift_avx_intrinsics;
#[cfg(target_arch = "x86_64")]
pub use xorshift_avx_intrinsics::xorshift_avx_intrinsics;

#[cfg(target_arch = "x86_64")]
mod xorshift_avx;
#[cfg(target_arch = "x86_64")]
pub use xorshift_avx::xorshift_avx;

#[cfg(target_arch = "x86_64")]
mod xorshift_avx_ss4;
#[cfg(target_arch = "x86_64")]
pub use xorshift_avx_ss4::xorshift_avx_ss4;

#[cfg(target_arch = "x86_64")]
mod xorshift_avx_ss8;
#[cfg(target_arch = "x86_64")]
pub use xorshift_avx_ss8::xorshift_avx_ss8;
