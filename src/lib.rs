
pub mod cumsum_f32{
    pub use cumsum_f32::*;
}
pub mod cumsum_f64{
    pub use cumsum_f64::*;
}
pub mod xorshift{
    pub use xorshift::*;
}
pub mod xorshiro256plus{
    pub use xorshiro256plus::*;
}

mod u64_to_f64;
pub use u64_to_f64::*;

mod sample;
pub use sample::sample;

mod sample_avx;
pub use sample_avx::sample_avx;

mod random;
pub use random::*;