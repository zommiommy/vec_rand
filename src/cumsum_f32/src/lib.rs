#![feature(asm)]

mod cumsum_f32_plain;
pub use cumsum_f32_plain::cumsum_f32_plain;

mod cumsum_f32_scan;
pub use cumsum_f32_scan::cumsum_f32_scan;

mod cumsum_f32_unrolled;
pub use cumsum_f32_unrolled::cumsum_f32_unrolled;


#[cfg(target_arch = "x86_64")]
mod cumsum_f32_sse_intrinsics;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f32_sse_intrinsics::cumsum_f32_sse_intrinsics;


#[cfg(target_arch = "x86_64")]
mod cumsum_f32_avx;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f32_avx::cumsum_f32_avx;

#[cfg(target_arch = "x86_64")]
mod cumsum_f32_avx_intrinsics;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f32_avx_intrinsics::cumsum_f32_avx_intrinsics;


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
extern { fn cumsum_super_scaler_simd(array: *mut f32, len: usize);}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn ccumsum(array: &mut [f32], len: usize){
    unsafe{
        cumsum_super_scaler_simd(   array.as_mut_ptr(), len)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
extern { fn prefix_sum_AVX256(array: *mut f32, len: usize); }

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn the_stackoverflow_thingy_256(array: &mut [f32], len: usize) {
    unsafe{
        prefix_sum_AVX256(   array.as_mut_ptr(), len)
    }
}

pub fn cumsum_f32(random_vec: &mut Vec<f32>) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse") {
            let l = random_vec.len();
            ccumsum(random_vec, l);
            return;
        }
    }
    cumsum_f32_unrolled(random_vec)
}