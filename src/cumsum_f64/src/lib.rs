#![feature(asm)]

mod cumsum_f64_plain;
pub use cumsum_f64_plain::cumsum_f64_plain;

mod cumsum_f64_scan;
pub use cumsum_f64_scan::cumsum_f64_scan;

mod cumsum_f64_unrolled;
pub use cumsum_f64_unrolled::cumsum_f64_unrolled;

#[cfg(target_arch = "x86_64")]
mod cumsum_f64_sse;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f64_sse::cumsum_f64_sse;

#[cfg(target_arch = "x86_64")]
mod cumsum_f64_sse_intrinsics;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f64_sse_intrinsics::cumsum_f64_sse_intrinsics;

mod cumsum_f64_sse_modifing;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f64_sse_modifing::cumsum_f64_sse_modifing;



#[cfg(target_arch = "x86_64")]
mod cumsum_f64_avx;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f64_avx::cumsum_f64_avx;

#[cfg(target_arch = "x86_64")]
mod cumsum_f64_avx_intrinsics;
#[cfg(target_arch = "x86_64")]
pub use cumsum_f64_avx_intrinsics::cumsum_f64_avx_intrinsics;


pub fn cumsum_f64(random_vec: &mut Vec<f64>){
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        // to debug
        // if is_x86_feature_detected!("avx2") {
        //     cumsum_f64_avx_intrinsics(random_vec);
        //     return;
        // }
        if is_x86_feature_detected!("sse") {
            cumsum_f64_sse_intrinsics(random_vec);
            return;
        }
    }
    cumsum_f64_unrolled(random_vec)
}