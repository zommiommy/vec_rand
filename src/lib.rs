#![feature(core_intrinsics)]
#![no_std]
//! Crate with various implementation of Pseudo-Random Number Generators.
//!
//! These implementations are in no way ment to be Cryptographically safe, Their
//! intended porpouse is to do MonteCarlo simulations and Random-Walks on graphs.
//!
//! # Benchmarks
//!
//! On my `Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz` I get the following timings:
//!
//! ```plain
//! test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//!
//! Running target/release/deps/test_cumsum_f32-d5cc7430807cdb72
//!
//! running 4 tests
//! test test_cumsum_f32                ... bench:      25,742 ns/iter (+/- 2,006)
//! test test_cumsum_f32_scan           ... bench:      30,146 ns/iter (+/- 1,987)
//! test test_cumsum_f32_sse_intrinsics ... bench:       5,592 ns/iter (+/- 711)
//! test test_cumsum_f32_unrolled       ... bench:      12,913 ns/iter (+/- 1,192)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
//!
//! Running target/release/deps/test_cumsum_f64-6b7deff0f832ff2d
//!
//! running 5 tests
//! test test_cumsum_f64                ... bench:     426,874 ns/iter (+/- 121,341)
//! test test_cumsum_f64_avx_intrinsics ... bench:   1,540,721 ns/iter (+/- 116,877)
//! test test_cumsum_f64_scan           ... bench:     545,887 ns/iter (+/- 122,878)
//! test test_cumsum_f64_sse_intrinsics ... bench:     172,347 ns/iter (+/- 64,432)
//! test test_cumsum_f64_unrolled       ... bench:     292,061 ns/iter (+/- 444,892)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out
//!
//! Running target/release/deps/test_gen_random_vec-dd6114be1bc6bb22
//!
//! running 3 tests
//! test test_std                  ... bench:     219,573 ns/iter (+/- 29,356)
//! test test_with_xorshift        ... bench:      28,741 ns/iter (+/- 2,694)
//! test test_with_xorshiro256plus ... bench:      27,060 ns/iter (+/- 2,646)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out
//!
//! Running target/release/deps/test_sampling-dfb7e71c37ebc1f2
//!
//! running 3 tests
//! test test_sample                ... bench:     215,570 ns/iter (+/- 317,531)
//! test test_sample_avx            ... bench:     428,251 ns/iter (+/- 259,277)
//! test test_weighted_index_sample ... bench:     450,141 ns/iter (+/- 534,151)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out
//!
//! Running target/release/deps/test_xorshift-438f22f403d678ed
//!
//! running 6 tests
//! test test_thread_rng              ... bench:          13 ns/iter (+/- 1)
//! test test_xorshift                ... bench:           2 ns/iter (+/- 0)
//! test test_xorshift_avx            ... bench:           3 ns/iter (+/- 0)
//! test test_xorshift_avx_intrinsics ... bench:          45 ns/iter (+/- 2)
//! test test_xorshift_avx_ss4        ... bench:           4 ns/iter (+/- 0)
//! test test_xorshift_avx_ss8        ... bench:           6 ns/iter (+/- 0)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out
//!
//! Running target/release/deps/test_xorshiro256plus-338a7678e125f597
//!
//! running 4 tests
//! test test_thread_rng              ... bench:          13 ns/iter (+/- 0)
//! test test_xorshiro256plus         ... bench:           2 ns/iter (+/- 0)
//! test test_xorshiro256plus_avx     ... bench:           3 ns/iter (+/- 0)
//! test test_xorshiro256plus_avx_ss4 ... bench:          11 ns/iter (+/- 1)
//! ```
//!
//! It's worth noticing that the """fastest""" prng is the xorshift_avx_ss4 that's generate
//! 16 u64 in 4 ns which means 250ps per u64 and 31.125ps per byte.
//!

#[cfg(feature="alloc")]
extern crate alloc;

pub mod cumsum_f32;
pub use cumsum_f32::*;

pub mod cumsum_f64;
pub use cumsum_f64::*;

pub mod xorshift;
pub use xorshift::*;

pub mod xorshiro256plus;
pub use xorshiro256plus::*;

mod u64_to_f64;
pub use u64_to_f64::*;

mod random;
pub use random::*;

mod gen_random_vec;
pub use gen_random_vec::*;

mod splitmix64;
pub use splitmix64::*;

mod gen_random_vec_f64;
pub use gen_random_vec_f64::*;

pub mod iter_suss;
pub use iter_suss::*;

// export the fastest implementation
pub use u64_to_f64::u64_to_f64_no_mul as u64_to_f64;

mod sample;
pub use sample::sample;
mod sample_plain;
pub use sample_plain::sample_plain;
#[cfg(all(target_arch = "x86_64"))]
mod sample_avx;
#[cfg(all(target_arch = "x86_64"))]
pub use sample_avx::sample_avx;
mod sample_modifing;
#[cfg(all(target_arch = "x86_64"))]
pub use sample_modifing::sample_modifing;

mod sample_f32;
pub use sample_f32::sample_f32;
pub use sample_f32::sample_from_cumsum;

mod sample_f32_adapt;
pub use sample_f32_adapt::sample_f32_adapt;

mod sample_uniform_biased;
pub use sample_uniform_biased::sample_uniform_biased;
mod sample_uniform_unbiased;
pub use sample_uniform_unbiased::sample_uniform_unbiased_nearly_divisionless;
pub use sample_uniform_unbiased::sample_uniform_unbiased_simple;
pub use sample_uniform_unbiased::sample_uniform_unbiased_simple as sample_uniform;

mod sample_k_distinct_uniform;
pub use sample_k_distinct_uniform::*;

#[cfg(feature="alloc")]
pub use gen_random_vec::gen_random_vec_1;

#[cfg(feature="alloc")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use gen_random_vec::gen_random_vec_4_1;


#[cfg(feature="std")]
pub fn gen_random_vec(size: usize, seed: u64) -> alloc::vec::Vec<u64> {
    extern crate std;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if std::is_x86_feature_detected!("avx2") {
            return gen_random_vec::gen_random_vec_4_1(size, seed);
        }
    }
    gen_random_vec::gen_random_vec_1(size, seed)
}
